# NanoTekSpice
NanoTekSpice is a logic simulator that builds a circuit from a configuration file.
A circuit is composed of chipsets that are linked together.

## Chipsets
Here is an example of what's inside a simple chipset, the **4081** :

<img width="440" height="440" alt="image" src="https://github.com/user-attachments/assets/955a794a-d76d-4fc2-a2f3-33d7332722cf" />

The 4081 is a little box that contains four AND gates, with two inputs and one output each.
The 4081 features 14 connectors :<br>
3 pin 1 and pin 2 are inputs of one of the AND gates (gate 1),<br>
3 pin 3 is the output of gate 1,<br>
3 pin 4 is the output of gate 2,<br>
3 pin 5 and pin 6 are inputs of gate 2,<br>
3 pin 7 (VSS) is for electrical purposes; we will ignore it in this project,<br>
3 pin 8 and pin 9 are inputs of gate 3,<br>
3 pin 10 is the output of gate 3,<br>
3 pin 11 is the output of gate 4,<br>
3 pin 12 and pin 13 are inputs of gate 4,<br>
3 pin 14 (VOD) is for electrical purposes; we will ignore it in this project<br>

## Architecture

<img width="5312" height="2384" alt="Bienvenue sur FigJam" src="https://github.com/user-attachments/assets/b951affc-5f0b-4538-8d46-b546ec931e45" />

## Installation

#### 1. Clone the repository
```bash
git clone git@github.com:EpitechPGE2-2025/G-OOP-400-PAR-4-1-tekspice-23.git
cd G-OOP-400-PAR-4-1-tekspice-23
```

#### 2. Compile
```bash
make
```

## Usage
#### File
Here is an example of a configuration file.

```env
.chipsets:
clock clk
input reset
4040 counter
output q1
output q2
output q3
output q4

.links:
counter:10 clk:1
counter:11 reset:1
counter:9 q1:1
counter:7 q2:1
counter:6 q3:1
counter:5 q4:1
```

The chipsets section is where you declare the components you will use for your circuits (input, output, and other components).
The links section is here to connect components.

#### Run

```bash
./nanotekspice 4040.nts
```

![4040_demo](https://github.com/user-attachments/assets/02592e8b-a954-4474-87b1-d343dec91fca)

## Contributing
@aludnier and @Rayan-ouer
