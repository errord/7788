
var fs = require("fs");

function readFile(path) {
    return fs.readFileSync(path, "utf-8");
}

function println(msg) {
    console.log(msg);
}
    

var MAX_PROBE = 6

function loadDict(dictpath) {
    var dict = readFile(dictpath);
    var lines = dict.split("\n");
    var wsdict = {};
    for (var i = 0; i < lines.length; i++) {
	var line = lines[i];
	var wordstruct = line.trim().split("\t");
	wordstruct = wordstruct.map(function(k) {
	    if (k.indexOf(",") > 0) {return k.substr(0, k.length-1)} else {return k}
	})
	if (wordstruct[0].length > 1) {
	    wsdict[wordstruct[0]] = wordstruct;
	}
    }
    return wsdict;
}

function isAlpha(word) {
    var re = /^[a-zA-Z]*$/g;
    if (!re.test(word)) {
        return false;  
    } else {
	return true;  
    }
}

function isDigit(word) {
    var re = /^[0-9一二三四五六七八九十百千万亿]*$/g;
    if (!re.test(word)) {
        return false;  
    } else {
	return true;  
    }
}

function isDigitOrAlpha(word) {
    var re = /^[0-9a-zA-Z一二三四五六七八九十百千万亿]*$/g;
    if (!re.test(word)) {
        return false;  
    } else {
	return true;  
    }
}

function extractDigitOrAlpha(words, idx) {
    var wordslen = words.length, digit = "";
    while (idx < wordslen) {
	var w = words[idx];
	if (isDigitOrAlpha(w)) {
	    digit += w
	    idx++;
	} else {
	    break;
	}
    }
    return [idx, digit]
}

function ws(wsdict, words) {
    var wordlist = [], idx = 0, word = "", wordslen = words.length
    while (idx < wordslen) {
	word = words.substr(idx, MAX_PROBE);
	// extract digit or alpha
	if (isDigitOrAlpha(word[0])) {
	    var l = extractDigitOrAlpha(words, idx);
	    idx = l[0], word = l[1];
	    wordlist.push(word);
	    continue;
	}
	while (word.length > 0) {
	    if (wsdict.hasOwnProperty(word)) {
		wordlist.push(word);
		break;
	    } else {
		word = word.substr(0, word.length-1);
	    }
	}
	if (!word) {
	    wordlist.push(words[idx]);
	    idx++;
	} else {
	    idx += word.length;
	}
    }
    return wordlist;
}

function currentTime() {
    return (new Date()).getTime();
}

function stringToBytes(str) {
  var ch, st, re = [];
  for (var i = 0; i < str.length; i++ ) {  
    ch = str.charCodeAt(i);  // get char   
    st = [];                 // set up "stack"  
    do {  
      st.push( ch & 0xFF );  // push byte to stack  
      ch = ch >> 8;          // shift value down by 1 byte  
    } while(ch);
    // add stack contents to result  
    // done because chars have "wrong" endianness  
    re = re.concat( st.reverse() );
  }
  // return an array of bytes  
  return re;
}

var starttime = currentTime();
var wsdict = loadDict("../data/Freq/word.dict");
println("load dict time: " + (currentTime() - starttime) + "ms");
var totalWordCount = 0;
var totalTextCount = 0;
starttime = currentTime();
var text = readFile("../data/news/news.sohunews.210806.txt.utf8");
var lines = text.split("\n");
var usetime = currentTime() - starttime;
console.log("open file time: %ss", usetime/1000);
//println(ws(wsdict, lines[153]));
for (var i = 0; i < lines.length; i++) {
    var line = lines[i];
    totalWordCount += (ws(wsdict, line)).length;
}
var usetime = currentTime() - starttime;
console.log("segment time: %ss", usetime/1000);

for (var i = 0; i < lines.length; i++) {
    var line = lines[i];
    totalTextCount += (stringToBytes(line)).length;
}

console.log("word text count: %s rate: %sKB/s", totalTextCount,
	    ((totalTextCount / (usetime/1000))/1000));
console.log("word total count: %s", totalWordCount);
