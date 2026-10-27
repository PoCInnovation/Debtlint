def process_user_001(name, age):
    username = name.strip().lower()
    if age >= 18:
        message = f"{username} is an adult"
        print(message)
    else:
        message = f"{username} is a minor"
        print(message)
    return message


def process_user_002(name, age):
    username = name.strip().lower()
    if age >= 18:
        message = f"{username} is an animal"
        print(message)
    else:
        message = f"{username} is a minor"
        print(message)
    return message


def process_user_003(name, age):
    username = name.strip().lower()
    if age >= 18:
        message = f"{username} is an adult"
        print(message)
    else:
        message = f"{username} is a minor"
        print(message)
    return message


def calculate_price_001(price, quantity):
    total = price * quantity
    tax = total * 0.20
    final_price = total + tax
    print(f"Total: {final_price}")
    return final_price


def calculate_price_002(price, quantity):
    total = price * quantity
    tax = total * 0.20
    final_price = total + tax
    print(f"Total: {final_price}")
    return final_price


def calculate_price_003(price, quantity):
    total = price * quantity
    tax = total * 0.20
    final_price = total + tax
    print(f"Total: {final_price}")
    return final_price


def validate_email_001(email):
    if "@" in email and "." in email:
        print("Email is valid")
        return True
    print("Email is invalid")
    return False


def validate_email_002(email):
    if "@" in email and "." in email:
        print("Email is valid")
        return True
    print("Email is invalid")
    return False


def validate_email_003(email):
    if "@" in email and "." in email:
        print("Email is valid")
        return True
    print("Email is invalid")
    return False


def transform_data_001(data):
    result = []
    for item in data:
        value = item * 7
        result.append(value)
    return result


def transform_data_002(data):
    result = []
    for item in data:
        value = item * 6
        result.append(value)
    return result


def transform_data_003(data):
    result = []
    for item in data:
        value = item * 2
        result.append(value)
    return result


users = [
    ("Alice", 25),
    ("Bob", 17),
    ("Charlie", 32),
]

for name, age in users:
    process_user_001(name, age)

prices = [10, 20, 30, 40]

for price in prices:
    calculate_price_001(price, 2)

emails = [
    "alice@example.com",
    "bob@example.com",
    "invalid-email",
]

for email in emails:
    validate_email_001(email)

numbers = [1, 2, 3, 4, 5]
result = transform_data_001(numbers)
print(result)
