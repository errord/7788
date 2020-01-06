#coding=utf-8

import os
import time

def iconv(path, filename):
    f = open(path + "/" + filename + ".utf8", "w")
    for line in open(path + "/" + filename).readlines():
        l =  line.decode("gb2312", 'ignore')
        f.write(l.encode("utf8"))
    f.close()

def iconv_path(path):
    for filename in os.listdir(path):
        stime = time.time()
        iconv("../data/news", filename)
        print "process %s use: %sms" % (filename, time.time() - stime)

if __name__ == '__main__':
    iconv_path("../data/news");
