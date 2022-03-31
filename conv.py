import base64
a = (3633393736383832383733353539383139363339393838303830383230393037).to_bytes(32, byteorder='big')
print(a)

b = 'Ub4/KDKclHxuvh6UqOZyNQ=='
c = b.encode('utf-8')
d = base64.decodebytes(c)
print(d)

b = 'AKNNODGUyFaoN/QlFZgiwlB8CNZzzHyO65mBUMuP9HWp/llH1t8B+WFg1zAunm9KONgWzhRkPuBZSLnz4BJQXA=='
c = b.encode('utf-8')
d = base64.decodebytes(c)
print(d)
