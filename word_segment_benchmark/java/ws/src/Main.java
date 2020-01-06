import java.io.BufferedReader;
import java.io.File;
import java.io.FileInputStream;
import java.io.InputStreamReader;
import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Main {

    private static int MAX_PROBE = 6;
    private static Map<String, List<String>> dictMap = new HashMap<>();
    private static Pattern numericAndAlphaPattern = Pattern.compile("^[0-9a-zA-Z十百千万亿]*");
    private static long startTime = 0;


    public static String ToDBC(String input) {
        char c[] = input.toCharArray();
        for (int i = 0; i < c.length; i++) {
            if (c[i] == '\u3000') {
                c[i] = ' ';
            } else if (c[i] > '\uFF00' && c[i] < '\uFF5F') {
                c[i] = (char) (c[i] - 65248);
            }
        }
        String returnString = new String(c);

        return returnString;
    }

    public static List<Object> extractDigitOrAlpha(String words, int idx) {
        List<Object> r = new LinkedList<>();
        String digit = "";

        while (idx < words.length()) {
            String word = words.substring(idx, idx+1);
            if (isNumericOrAlpha(word)) {
                digit += word;
                idx++;
            } else {
                break;
            }
        }
        r.add(0, idx);
        r.add(1, digit);
        return r;
    }

    public static boolean isNumericOrAlpha(String str){
        Matcher isNumOrAlpha = numericAndAlphaPattern.matcher(str);
        if( !isNumOrAlpha.matches() ){
            return false;
        }
        return true;
    }

    private static void loadDict(String dictPath) {
        try {
            String encoding="utf8";
            File file=new File(dictPath);
            if(file.isFile() && file.exists()){ //判断文件是否存在
                InputStreamReader read = new InputStreamReader(
                        new FileInputStream(file), encoding);//考虑到编码格式
                BufferedReader bufferedReader = new BufferedReader(read);
                String lineTxt = null;
                while((lineTxt = bufferedReader.readLine()) != null) {
                    List<String> lines = Arrays.asList(lineTxt.split("\t"));
                    for (int i = 0; i < lines.size(); i++) {
                        String line = lines.get(i);
                        if (line.contains(",")) {
                            lines.set(i, line.substring(0, line.length() - 1));
                        }
                    }
                    dictMap.put(lines.get(0), lines);
                }
                read.close();
            }else{
                System.out.println("找不到指定的文件");
            }
        } catch (Exception e) {
            System.out.println("读取文件内容出错");
            e.printStackTrace();
        }
    }



    private static List<String> ws(String words) {
        List<String> wordList = new LinkedList<>();
        String word;
        int idx = 0;
        words = ToDBC(words);
        while (idx < words.length()) {
            word = words.substring(idx, idx + Math.min(MAX_PROBE, words.length() - idx));
            /* extracr digit */
            if (isNumericOrAlpha(word.substring(0, 1))) {
                List<Object> r = extractDigitOrAlpha(words, idx);
                idx = (int)(r.get(0));
                word = (String)(r.get(1));
                wordList.add(word);
                continue;
            }
            while (!word.isEmpty()) {
                if (dictMap.getOrDefault(word, null) != null) {
                    wordList.add(word);
                    break;
                } else {
                    word = word.substring(0, word.length()-1);
                }
            }

            if (word.isEmpty()) {
                wordList.add(String.valueOf(words.charAt(idx)));
                idx++;
            } else {
                idx += word.length();
            }
        }
        return wordList;
    }

    public static String wordSegment(String words) {
        List<String> wordsList = ws(words);
        return String.join(" ", wordsList);
    }

    private static void wordSegmentByTextFile(String filePath) {
        int totalTextCount = 0;
        int totalWordCount = 0;
        try {
            String encoding = "utf8";
            File file=new File(filePath);
            if(file.isFile() && file.exists()){ //判断文件是否存在
                InputStreamReader read = new InputStreamReader(
                        new FileInputStream(file), encoding);//考虑到编码格式
                BufferedReader bufferedReader = new BufferedReader(read);
                String lineTxt = null;
                startTime = System.currentTimeMillis();
                //int i = 0;
                while((lineTxt = bufferedReader.readLine()) != null) {
                    totalTextCount += lineTxt.getBytes().length;
                    totalWordCount += ws(lineTxt).size();
                    /*
                    if (i == 153)
                        System.out.println(ws(lineTxt));
                    i++;
                    */
                }
                long useTime = System.currentTimeMillis() - startTime;
                System.out.println("segment  time: " + useTime/1000.0 + "s");
                System.out.println("word text count: " + totalTextCount + " rate: " + (totalTextCount/((int)(useTime/1000.0)))/1000.0 + "KB/s");
                System.out.println("word total count: " + totalWordCount);
                read.close();
            }else{
                System.out.println("找不到指定的文件");
            }
        } catch (Exception e) {
            System.out.println("读取文件内容出错");
            e.printStackTrace();
        }
    }

    public static void main(String[] args) {
        startTime = System.currentTimeMillis();
	String path = "/Users/d/Work/my/cutword/word_segment";
        loadDict(path + "/data/Freq/word.dict");
        System.out.println("load dict time: " + String.valueOf(System.currentTimeMillis() - startTime) + "ms");
        wordSegmentByTextFile(path + "/data/news/news.sohunews.210806.txt.utf8");
    }
}
