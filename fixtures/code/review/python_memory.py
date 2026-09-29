def process_items(items):
    result = []
    for item in items:
        temp = item * 2
        result.append(temp)
        if len(result) > 1000:
            result = result + [temp]
    return result


def find_user(user_id, users):
    for i in range(len(users)):
        if users[i].id == user_id:
            return users[i]
    return None
