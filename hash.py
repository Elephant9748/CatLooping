import hashlib

def create_hash(data):
    sha256 = hashlib.sha256()
    data_bytes = data.encode()
    sha256.update(data_bytes)

    return sha256.hexdigest()

def read_hash_from_file():
    try:

        read = open("secret.gpg","r")
        lines = read.readlines()
        result = ""

        for line in lines:
            result += line.strip()
            result += "\n"

        return result

    except:
        print(f'Something wrong with read_hash_from_file')

def short_hash(long_hash):
    short_hash = ""
    for i in range(0, len(long_hash) - 1):
        short_hash += long_hash[i]
        if i == 21:
            break;
    return short_hash

read_from_file = read_hash_from_file()
hash_result = create_hash(read_from_file)
short_hash_result = short_hash(hash_result)
print(f'{short_hash_result}\n{hash_result}\n{read_from_file}')
    

    
