# Pourquoi le BPE ? (candidats ≠ score)

Document de clarification pour l’équipe : à quoi sert vraiment le BPE, comment on choisit quoi comparer, comment marche la fenêtre, et un exemple complet bout-en-bout.

**Contrainte produit** : Debtlint est **language-agnostic**.  
Pas de parseur de langage, pas de renommage de variables (`a` → `$1`), pas d’AST.  
Le “nettoyage” en amont = surtout **quels fichiers on prend** + un **clean léger** (ex. commentaires `#`, bruit de surface).

Voir aussi : [bpe.md](./bpe.md), [similarity-engine.md](./similarity-engine.md), [hyperparams.md](./hyperparams.md).

---

## La question

> Si on parse tout en un langage abstrait, puis on compare chaque fonction entre elles, à quoi sert le BPE ?

Et le corollaire :

> Deux fonctions avec 2 tokens identiques et 3 différents = 40 % de similarité en tokens.  
> Mais en caractères, ça peut être 95 %. Donc le score token est faux.

---

## Réponse courte

| Rôle | Qui le fait | BPE utile ? |
|------|-------------|-------------|
| Trouver **quoi comparer** (candidats) | BPE + occurrences | **Oui** |
| Dire **à quel %** c’est similaire (score) | Métrique sur fenêtre de tokens | **Pas le BPE seul** |

> **BPE ≠ juge de similarité.**  
> **BPE = index / filtre pour trouver des paires intéressantes.**

- BPE = moteur de recherche (« où regarder ? »)
- Score = mesure fine (« à quel point c’est proche ? »)

---

## Ce que Debtlint nettoie vraiment (agnostique)

Pas ça :

```
❌ détecter les variables
❌ renommer a/b → $1/$2
❌ parser fn / class / fonction
❌ IR / AST par langage
```

Oui ça (léger, language-agnostic) :

```
✅ quels fichiers / dossiers inclure ou exclure
✅ retirer les commentaires type # ...
✅ éventuellement un clean de surface (lignes vides excessives, etc.)
✅ ensuite : texte → tokens BPE
```

Donc dans l’exemple `add` / `x+y`, **`a` et `x` restent différents**.  
Le moteur doit vivre avec ça — d’où l’intérêt de ne pas scorer bêtement “nb de tokens égaux”.

---

## Comment on choisit **où** comparer en priorité

On ne compare pas tout × tout. On priorise.

### 1. Filtrer les merges candidats

On ignore l’alphabet de base (`a`, `{`, espace…).  
On garde les **merges** qui :

| Critère | Pourquoi |
|---------|----------|
| ≥ 2 occurrences | sinon pas de paire à comparer |
| (idéal) multi-locations | 2 endroits différents = duplication potentielle |
| merges « assez gros » en premier | un gros bloc commun > un petit `im` |

### 2. Priorité entre candidats

Ordre suggéré (MVP) :

```
1. Merges les plus fréquents d’abord
2. Puis merges avec le plus de locations distinctes
3. (plus tard) merges les plus « profonds » / longs
```

Exemple :

```
Merge #150  "fn add(...) {"   → 4 occurrences  → prioritaire
Merge #98   "i3"              → 2 occurrences  → plus bas
Merge #50   "a" (symbole)     → ignoré
```

### 3. Construire les paires

Pour un merge avec locations `[L1, L2, L3]` :

```
paires prioritaires = (L1,L2), (L1,L3), (L2,L3)
```

On score **seulement ces paires**, pas le corpus entier.

---

## Comment on compare

Une fois une paire `(L1, L2)` choisie :

```
1. Extraire une fenêtre de tokens autour de chaque offset

2. Calculer une distance sur ces fenêtres
   → Levenshtein / LCS sur &[Token]

3. (mieux) pondérer par la taille réelle des tokens
   → un gros merge compte plus qu’un symbole 'a'

4. Convertir en %
   → score = similarité pondérée / longueur

5. Classer
   → 90–100 / 70–90 / 60–70 / ignorer < 60
```

**Important** : on compare la **fenêtre**, pas « le nombre de merges partagés ».  
Et on ne suppose **pas** de renommage de variables.

---

## La fenêtre : hardcodée ?

### MVP : oui, hyperparam fixe

```
window_size = 32   // tokens, pas caractères
```

Autour de l’**offset** (index dans la séquence BPE) :

```
séquence : [ ... tokens ... ]
                 ↑ offset

fenêtre = sequence[offset - half .. offset + half]
bornée au début / fin du fichier
```

Réglage CLI, comme `vocab_size` :

```bash
debtlint ... --window-size 32 --min-score 0.75
```

| Paramètre | Défaut suggéré | Rôle |
|-----------|----------------|------|
| `window_size` | `32` | taille de la fenêtre en tokens |
| `min_score` | `0.75` | seuil d’affichage |
| `min_merge_occurrences` | `2` | merge candidat minimum |

### Plus tard (pas obligatoire au PoC)

| Variante | Idée |
|----------|------|
| Fenêtre adaptative | s’étendre jusqu’à un délimiteur générique (`}`, ligne vide…) sans parser le langage |
| Multi-échelle | tester 16 / 32 / 64, garder le meilleur score |

**Recommandation PoC** : flag `--window-size`, pas de découpage “par fonction” (ça casserait l’agnosticité).

---

## Exemple complet : les deux `add`

### Entrée source

```rust
// fichier A
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// fichier B
fn add(a: i32, b: i32) -> i32 {
    x + y
}
```

---

### Étape 0 — Clean agnostique (léger)

Exemple de ce qui se passe vraiment :

```
- on décide d’inclure A.rs et B.rs (pas node_modules/, pas .git/, …)
- on retire les commentaires # ...
- on ne touche PAS aux noms de variables
```

Résultat après clean (schéma) : le code utile reste, `a` / `x` / `b` / `y` **inchangés**.

```
A: fn add(a: i32, b: i32) -> i32 { a + b }
B: fn add(a: i32, b: i32) -> i32 { x + y }
```

Pas de `$1` / `$2`. Debtlint ne “comprend” pas que ce sont des variables.

---

### Étape 1 — BPE (tokens + merges)

Chaque caractère / symbole de base devient un token, puis le BPE merge les paires fréquentes.

Version simplifiée (ids fictifs) :

```
Alphabet de base (extrait) :
  'f'=10 'n'=11 ' '=12 'a'=13 ... '+'=40 ...

Texte A découpé :
  f n   a d d ( a :   i 3 2 ,   b :   i 3 2 ) ... {   a   +   b }
```

Comme la **signature** apparaît 2 fois (A et B), le BPE crée des merges dessus :

```
Merge #120 : "fn"
Merge #130 : "add"
Merge #140 : "i32"
Merge #150 : "fn add(a: i32, b: i32) -> i32 {"   ← gros merge commun
```

Le corps :
- `a + b` n’apparaît qu’**une fois** → pas de gros merge
- `x + y` n’apparaît qu’**une fois** → pas de gros merge

```
Vocabulaire (schéma) :
  Symboles 0..96
  ...
  Merge #150 {
    pair: (...),
    occurrences: [
      { path: "A.rs", offsets: [0] },   // offset = index token du début du merge
      { path: "B.rs", offsets: [0] }
    ]
  }
```

**Offset** = index dans la séquence de tokens où commence la paire mergée  
(pas une position caractère dans le fichier source).

---

### Étape 2 — Engine : choix prioritaire

```
Candidat prioritaire = Merge #150
  → 2 locations : A@0 et B@0
  → 1 paire à scorer : (A@0, B@0)
```

On ne compare pas toute la codebase. On compare **cette paire**.

---

### Étape 3 — Fenêtre (on ne “détecte” pas la fin)

`window_size` = hyperparam fixe (ex. 32, ou 6 pour l’exemple réduit ci-dessous).

**Point important** : on ne sait **pas** où s’arrête le bloc.

Le merge dit seulement « le pattern commence à cet offset ».  
La fenêtre prend bêtement les N tokens **autour / après** cet offset dans la séquence.  
Le corps (`a + b`, `x + y`, `}`) entre **parce qu’il est à côté**, pas parce qu’il est commun, et pas parce qu’on a parsé une fonction.

```
séquence A :  [ #150 ][ a ][ + ][ b ][ } ][ ...suite... ]
              ↑ offset
              |←── fenêtre de N tokens ──→|
```

On ne s’arrête pas “à `}`” : on s’arrête à **N tokens**.

---

### Étape 4 — Exemple concret de fenêtres (ids numériques)

Deux fenêtres issues du même candidat :

```
A:  156  134  157  178  189  190
B:  156  134  160  170  189  190
```

Visuel :

```
A: [156][134][157][178][189][190]
B: [156][134][160][170][189][190]
    ████ ████ ░░░░ ░░░░ ████ ████
    commun     diff       commun
```

Lecture :
- `156 134` = début commun (souvent le gros merge / pattern candidat)
- `157 178` vs `160 170` = milieu différent (ex. `a+b` vs `x+y`)
- `189 190` = fin encore commune (ex. `}` + suite coincée dans la fenêtre)

Ces tokens du milieu / de la fin ne sont **pas choisis** : ils sont juste dans la coupe fixe.

---

### Étape 5 — Le piège du score “nb de tokens”

Si on compte bêtement « tokens égaux / total » :

```
6 tokens :
  4 identiques (156, 134, 189, 190)
  2 différents (157/160, 178/170)

→ 4/6 ≈ 67 %
```

Ou pire, si on ne regarde que “merges partagés” / petits tokens du corps → on peut descendre vers **40–60 %** alors que la structure commune est énorme.

**Mauvais score** = chaque token compte pareil, alors qu’un gros merge ≠ un caractère.

C’est la remarque de Loan — valide dans un monde **agnostique** (pas de rename de variables).

---

### Étape 6 — Similarité (façons compatibles agnostiques)

#### Option A — Levenshtein simple sur tokens

```
A: 156 134 157 178 189 190
B: 156 134 160 170 189 190
           ↑   ↑
         replace replace

distance = 2
max(len) = 6
score = 1 - 2/6 ≈ 66.7 %   → bande Moyenne (60–70)
```

Simple, mais biaisé.

#### Option B — pondérer par la taille réelle du token (recommandé)

Sans parser le langage : on décode un token via le vocab → on connaît sa **longueur en caractères**.

Exemple fictif de tailles :

| Token | Taille (chars) | Rôle |
|-------|----------------|------|
| 156 | 20 | gros merge commun |
| 134 | 8 | merge / motif commun |
| 157 / 160 | 1 | petit symbole différent |
| 178 / 170 | 1 | petit symbole différent |
| 189 | 5 | commun |
| 190 | 5 | commun |

```
masse totale   ≈ 20+8+1+1+5+5 = 40
masse différente ≈ 1+1 = 2
score ≈ 1 - 2/40 = 95 %   → bande Critique (90–100)
```

Visuel pondéré :

```
A: [156========][134===][157][178][189=][190=]
B: [156========][134===][160][170][189=][190=]
    ████████████ ██████  ░░   ░░  ████  ████
         gros commun      petit diff    commun

non pondéré : 4/6 ≈ 67 %
pondéré     : ~95 %   (le milieu différent pèse peu)
```

→ toujours **agnostique**, mais on corrige le biais “gros token vs petit token”.

#### Et si le milieu était un gros merge différent ?

```
A: 156 134 200 189 190     # 200 = gros bloc
B: 156 134 300 189 190     # 300 = autre gros bloc
```

Là, même pondéré, le score **chute** — et c’est voulu : vrai écart structurel.

#### Ce qu’on ne fait pas

```
❌ renommer a/b et x/y en $1/$2
❌ détecter la fin de fonction pour “bien” découper
❌ s’arrêter à } parce que c’est du Rust
```

---

### Étape 7 — Rendu / rapport

Avec option B sur l’exemple numérique :

```
[CRITIQUE] ~95%
  A.rs offset=...
  B.rs offset=...
  candidat: merge lié à 156 / pattern commun
  fenêtre: 6 tokens (ex. réduit ; en vrai souvent 32)
```

Bande :

| Score | Bande |
|-------|-------|
| 90–100 | Critique |
| 70–90 | Élevée |
| 60–70 | Moyenne |
| &lt; 60 | Ignoré |

---

## Schéma récap de l’exemple

```
Source
  A: fn add(a,b){ a+b }
  B: fn add(a,b){ x+y }
        │
        ▼
Clean agnostique
  choix des fichiers + strip commentaires #
  (pas de rename de variables)
        │
        ▼
BPE
  gros merge commun (ex. 156) aux offsets A et B
  corps rare → pas de gros merge
        │
        ▼
Engine (priorité)
  candidat → paire (A@offset, B@offset)
        │
        ▼
Fenêtre FIXE (ex. 6 tokens)
  A: 156 134 157 178 189 190
  B: 156 134 160 170 189 190
  (on ne détecte PAS la fin de fonction)
        │
        ▼
Score
  PAS: 4/6 tokens = 67% comme vérité absolue
  OUI: Levenshtein pondéré par taille → ~95%
        │
        ▼
Rapport
  [CRITIQUE] A.rs ↔ B.rs
```

---

## Réponses directes

### « Comment on compare ? »
Fenêtre de tokens autour des offsets candidats + distance (idéalement **pondérée par taille** des tokens).

### « Comment on choisit où comparer en priorité ? »
Merges fréquents, multi-locations, gros patterns d’abord → paires d’occurrences de ces merges.

### « La fenêtre, on hardcode ? »
Oui au MVP : `window_size` (ex. 32), exposé en flag.  
On coupe à N tokens — on ne s’arrête pas “à `}`”.

### « Pourquoi la fenêtre contient le corps différent ? »
Parce qu’il est **à côté** de l’offset dans la séquence.  
Pas parce qu’il est commun, pas parce qu’on a compris la fonction.

### « On abstrait les variables ? »
**Non.** Debtlint est agnostique. Clean = fichiers inclus + commentaires / surface.  
`a` et `x` restent différents ; le score doit gérer ça (pondération).

### « Pourquoi le BPE alors ? »
Parce qu’il a trouvé les deux offsets sans comparer tout le repo.  
Le BPE propose la paire ; le score décide le %.

---

## Phrase pour l’équipe

> On est d’accord : le % de tokens BPE non pondéré n’est pas une bonne mesure.  
> Debtlint reste language-agnostic : pas de rename de variables, juste un clean léger (fichiers + commentaires).  
> Le BPE sert à prioriser où comparer (merges fréquents + offsets).  
> La fenêtre est un hyperparam (`window_size`) : coupe fixe, pas de détection de fin de fonction.  
> Exemple : `156 134 157 178 189 190` vs `156 134 160 170 189 190` → 2 diffs au milieu ; non pondéré ~67 %, pondéré ~95 % si le milieu est petit.  
> Le score final se fait sur cette fenêtre, idéalement pondéré par la taille réelle des tokens.
