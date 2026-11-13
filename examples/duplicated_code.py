def process_user_001(name, age, country, newsletter):
    username = name.strip().lower()
    country_code = country.strip().upper()
    wants_newsletter = bool(newsletter)
    if age >= 18:
        message = f"{username} ({country_code}) is an adult"
        print(message)
        status = "adult"
        can_purchase = True
    else:
        message = f"{username} ({country_code}) is a minor"
        print(message)
        status = "minor"
        can_purchase = False
    audit = {
        "user": username,
        "age": age,
        "status": status,
        "country": country_code,
        "newsletter": wants_newsletter,
        "can_purchase": can_purchase,
    }
    print(f"audit={audit}")
    if wants_newsletter and can_purchase:
        print(f"subscribe:{username}@mail.local")
    return message


def process_user_002(name, age, country, newsletter):
    username = name.strip().lower()
    country_code = country.strip().upper()
    wants_newsletter = bool(newsletter)
    if age >= 18:
        message = f"{username} ({country_code}) is an adult"
        print(message)
        status = "adult"
        can_purchase = True
    else:
        message = f"{username} ({country_code}) is a minor"
        print(message)
        status = "minor"
        can_purchase = False
    audit = {
        "user": username,
        "age": age,
        "status": status,
        "country": country_code,
        "newsletter": wants_newsletter,
        "can_purchase": can_purchase,
    }
    print(f"audit={audit}")
    if wants_newsletter and can_purchase:
        print(f"subscribe:{username}@mail.local")
    return message


def process_user_003(name, age, country, newsletter):
    username = name.strip().lower()
    country_code = country.strip().upper()
    wants_newsletter = bool(newsletter)
    if age >= 18:
        message = f"{username} ({country_code}) is an adult"
        print(message)
        status = "adult"
        can_purchase = True
    else:
        message = f"{username} ({country_code}) is a minor"
        print(message)
        status = "minor"
        can_purchase = False
    audit = {
        "user": username,
        "age": age,
        "status": status,
        "country": country_code,
        "newsletter": wants_newsletter,
        "can_purchase": can_purchase,
    }
    print(f"audit={audit}")
    if wants_newsletter and can_purchase:
        print(f"subscribe:{username}@mail.local")
    return message


def calculate_price_001(price, quantity, discount, vip):
    total = price * quantity
    if discount > 0:
        total = total - (total * discount)
    if vip:
        total = total * 0.95
    tax = total * 0.20
    shipping = 5.0 if total < 50 else 0.0
    insurance = 1.5 if total > 100 else 0.0
    final_price = total + tax + shipping + insurance
    print(f"subtotal={total}")
    print(f"tax={tax}")
    print(f"shipping={shipping}")
    print(f"insurance={insurance}")
    print(f"vip={vip}")
    print(f"Total: {final_price}")
    return final_price


def calculate_price_002(price, quantity, discount, vip):
    total = price * quantity
    if discount > 0:
        total = total - (total * discount)
    if vip:
        total = total * 0.95
    tax = total * 0.20
    shipping = 5.0 if total < 50 else 0.0
    insurance = 1.5 if total > 100 else 0.0
    final_price = total + tax + shipping + insurance
    print(f"subtotal={total}")
    print(f"tax={tax}")
    print(f"shipping={shipping}")
    print(f"insurance={insurance}")
    print(f"vip={vip}")
    print(f"Total: {final_price}")
    return final_price


def calculate_price_003(price, quantity, discount, vip):
    total = price * quantity
    if discount > 0:
        total = total - (total * discount)
    if vip:
        total = total * 0.95
    tax = total * 0.20
    shipping = 5.0 if total < 50 else 0.0
    insurance = 1.5 if total > 100 else 0.0
    final_price = total + tax + shipping + insurance
    print(f"subtotal={total}")
    print(f"tax={tax}")
    print(f"shipping={shipping}")
    print(f"insurance={insurance}")
    print(f"vip={vip}")
    print(f"Total: {final_price}")
    return final_price


def validate_email_001(email):
    cleaned = email.strip().lower()
    if "@" not in cleaned or "." not in cleaned:
        print("Email is invalid")
        return False
    local, _, domain = cleaned.partition("@")
    if not local or not domain or " " in cleaned:
        print("Email is invalid")
        return False
    if domain.startswith(".") or domain.endswith("."):
        print("Email is invalid")
        return False
    print("Email is valid")
    print(f"local={local}")
    print(f"domain={domain}")
    return True


def validate_email_002(email):
    cleaned = email.strip().lower()
    if "@" not in cleaned or "." not in cleaned:
        print("Email is invalid")
        return False
    local, _, domain = cleaned.partition("@")
    if not local or not domain or " " in cleaned:
        print("Email is invalid")
        return False
    if domain.startswith(".") or domain.endswith("."):
        print("Email is invalid")
        return False
    print("Email is valid")
    print(f"local={local}")
    print(f"domain={domain}")
    return True


def validate_email_003(email):
    cleaned = email.strip().lower()
    if "@" not in cleaned or "." not in cleaned:
        print("Email is invalid")
        return False
    local, _, domain = cleaned.partition("@")
    if not local or not domain or " " in cleaned:
        print("Email is invalid")
        return False
    if domain.startswith(".") or domain.endswith("."):
        print("Email is invalid")
        return False
    print("Email is valid")
    print(f"local={local}")
    print(f"domain={domain}")
    return True


users = [
    ("Alice", 25, "fr", True),
    ("Bob", 17, "us", False),
    ("Charlie", 32, "de", True),
]

for name, age, country, newsletter in users:
    process_user_001(name, age, country, newsletter)

for price in [10, 20, 30, 40]:
    calculate_price_001(price, 2, 0.1, False)

for email in ["alice@example.com", "bob@example.com", "invalid-email"]:
    validate_email_001(email)
