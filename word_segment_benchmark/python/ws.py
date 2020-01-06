#coding=utf-8

import time
import os

MAX_PROBE = 6
ws_dict = {}

def strQ2B(ustring):
    """全角转半角"""
    rstring = ""
    for uchar in ustring:
        inside_code = ord(uchar)
        if inside_code == 12288:                              #全角空格直接转换            
            inside_code = 32 
        elif (inside_code >= 65281 and inside_code <= 65374): #全角字符（除空格）根据关系转化
            inside_code -= 65248

        rstring += unichr(inside_code)
    return rstring
    
def strB2Q(ustring):
    """半角转全角"""
    rstring = ""
    for uchar in ustring:
        inside_code = ord(uchar)
        if inside_code == 32:                                 #半角空格直接转化                  
            inside_code = 12288
        elif inside_code >= 32 and inside_code <= 126:        #半角字符（除空格）根据关系转化
            inside_code += 65248

        rstring += unichr(inside_code)
    return rstring                                                                                                                                

def isalpha(word):
    if word.lower() in ("a", "b", "c", "d", "e", "f", "j", "h", "i", "g", "k", "l", "m", "n",
                "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"):
        return True
    return False

def load_dict(dict_path):
    for line in open(dict_path).readlines():
        word_struct = map(lambda x: x[:-1] if ',' in x else x.decode("utf8"), line.strip().split("\t"))
        if word_struct[0]:
            ws_dict[word_struct[0]] = word_struct

def extract_digit_or_alpha(words, idx):
    words_len = len(words)
    digit = ""
    while idx < words_len:
        w = words[idx]
        if w.isdigit() or w.isnumeric() or isalpha(w):
            digit += w
            idx += 1
        else:
            break
    return idx, digit

def is_digit_or_alpha(word):
    return word.isdigit() or word.isnumeric() or isalpha(word)

def ws(words):
    word_list = []
    idx = 0
    word = ""
    words = strQ2B(words.decode("utf8"))
    words_len = len(words)
    while idx < words_len:
        word = words[idx:idx+MAX_PROBE]
        # extracr digit
        if is_digit_or_alpha(word[0]):
            idx, word = extract_digit_or_alpha(words, idx)
            word_list.append(word)
            continue
        while word:
            if word in ws_dict:
                word_list.append(word)
                break
            else:
                word = word[:-1]
        if not word:
            word_list.append(words[idx])
            idx += 1
        else:
            idx += len(word)
    return word_list

def word_segment(words):
    word_list = ws(words)
    return ' '.join(word_list), len(word_list)

def doc_parse():
    pass

if __name__ == '__main__':
    start_time = time.time()
    load_dict("../data/Freq/word.dict")
    print "load dict time: %sms" % ((time.time() - start_time) * 1000)
    total_word_count = 0
    total_text_count = 0
    start_time = time.time()
    '''
    lll = open("../data/news/news.sohunews.210806.txt.utf8").readlines()
    c =  ws(lll[153])
    for i in c:
        print i
    0/0
    '''
    for line in open("../data/news/news.sohunews.210806.txt.utf8").readlines():
        total_text_count += len(line)
        total_word_count += len(ws(line))
    use_time = time.time() - start_time
    print "segment  time: %ss" % (use_time)
    print "word text count: %s  rate: %sKB/s" % (total_text_count, (total_text_count / use_time) / 1000)
    print "word total count: %s" % total_word_count

