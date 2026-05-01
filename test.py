import hashlib
import sys
import os

_dir = os.path.dirname(os.path.abspath(__file__))
words_raw = open(os.path.join(_dir, 'derive-bip-39-seed/wordlists/english.txt')).read().strip().splitlines()
wordlist = words_raw

phrases = [
    'labor great follow frame drip auction vanish nut load enemy coconut used',
    'tube tired cage message bar language sock trap speak lonely brief uncover',
    'load coconut frame follow great auction used nut drip labor enemy vanish',
    'drip follow coconut enemy nut labor vanish frame used load great auction',
    'frame drip auction used labor enemy nut vanish load coconut great follow',
    'auction enemy frame load vanish great follow nut used drip labor coconut',
    'vanish follow used enemy labor great drip nut load frame coconut auction',
    'frame load follow great drip used enemy auction nut labor vanish coconut',
    'follow auction load used enemy frame labor drip nut vanish great coconut',
    'drip frame used auction coconut vanish labor nut follow great load enemy',
    'follow auction coconut labor used vanish load nut frame great drip enemy',
    'enemy follow frame vanish drip great load nut used coconut labor auction',
    'follow vanish drip frame labor great load auction used enemy coconut nut',
    'follow vanish great load nut auction used enemy drip frame coconut labor',
    'drip coconut nut used labor enemy load vanish great frame follow auction',
    'enemy great auction drip nut frame labor vanish used load coconut follow',
    'used load coconut nut enemy drip labor frame vanish great follow auction',
    'great drip coconut nut frame used follow auction load enemy vanish labor',
    'coconut labor load enemy auction follow vanish drip nut great frame used',
    'load coconut great vanish enemy used auction nut frame drip labor follow',
    'follow enemy frame used drip nut load coconut vanish great auction labor',
    'used frame enemy nut great drip coconut follow vanish labor load auction',
    'labor nut load vanish used enemy great follow drip coconut frame auction',
    'coconut used frame follow drip labor enemy auction vanish nut great load',
    'great vanish frame enemy follow used load labor nut drip auction coconut',
    'labor load vanish drip auction frame used nut enemy great coconut follow',
    'frame drip used load follow great vanish coconut labor enemy nut auction',
    'load vanish labor drip frame nut used enemy auction follow coconut great',
    'drip nut labor frame vanish enemy coconut auction follow great load used',
    'great auction follow vanish labor frame nut drip enemy used coconut load',
    'great drip load labor auction nut coconut follow used vanish enemy frame',
    'vanish coconut drip great follow used nut frame auction labor load enemy',
    'enemy load coconut vanish frame auction labor drip nut used follow great',
    'vanish great drip frame enemy labor nut coconut load auction follow used',
    'vanish drip follow load labor auction used frame enemy coconut nut great',
    'coconut drip enemy used vanish great nut labor frame load auction follow',
    'follow used labor frame coconut enemy auction vanish drip great load nut',
    'drip labor frame vanish great enemy used nut follow auction load coconut',
    'frame labor follow auction used vanish nut coconut load enemy great drip',
    'vanish great load drip enemy coconut frame labor follow nut used auction',
    'enemy used great follow drip load nut labor coconut frame auction vanish',
    'great frame used load drip vanish auction enemy coconut nut labor follow',
    'drip used frame great vanish follow nut enemy coconut auction labor load',
    'great coconut auction used enemy follow drip vanish labor frame nut load',
    'nut coconut great auction follow enemy load frame labor drip vanish used',
    'follow nut load enemy used coconut drip frame auction labor vanish great',
    'enemy load follow coconut drip frame great labor vanish auction nut used',
    'great auction drip enemy load follow coconut vanish used frame labor nut',
]

def check(phrase):
    ws = phrase.split()
    try:
        indices = [wordlist.index(w) for w in ws]
    except ValueError as e:
        return f'WORD NOT FOUND: {e}'
    bits = 0
    for idx in indices:
        bits = (bits << 11) | idx
    # 132 bits total for 12 words: 128 entropy + 4 checksum
    checksum_bits = bits & 0xF
    entropy_bits = bits >> 4
    entropy = entropy_bits.to_bytes(16, 'big')
    digest = hashlib.sha256(entropy).digest()
    expected = digest[0] >> 4
    return 'VALID' if checksum_bits == expected else 'invalid'

for p in phrases:
    print(f'{check(p):7s}  {p}')