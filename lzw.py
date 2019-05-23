# ========== Introduction ========== #
''' LZW - Compression
    Program to compress file with TXT , TIF/TIFF, and GIF extension 
    using LZW Compression algorithm

    Pandyaka Aptanagi / 13517003 '''

# ========= Importing some utils ========= #
import sys
import base64
import string
from io import StringIO

# ========== Read External File ========= #
def readFile(filename) :

    # Open file based on its extension
    # Then return it as a string
    temp = ''
    if (filename[1] == "txt") :
        with open("{}.txt".format(filename[0])) as f :
            temp = f.read()
    else :
        with open("{}.{}".format(filename[0],filename[1]),"rb") as f :
            temp = str(base64.b64encode(f.read()))

    return temp

# =========== Write External File after Compression ========= #
def writeCompressFile(text,filename) :

    # Param : text and filename
    # Output : file with .txt extension, and contains text
    with open("{}.txt".format(filename[0]),"w+") as f :
        f.write(filename[1] + " file " + text)

# =========== Write External File after deCompression ========= #
def writeDecompressFile(text,filename,ex) :

    # Param : text and filename
    # Output : file with .ex extension
    if (ex == "txt") :
        with open("{}.txt".format(filename[0]),"w+") as f :
            f.write(text)
    else :
        out = base64.b64decode(text[2:-1])
        with open("{}.{}".format(filename[0],ex),"wb") as f :
            f.write(out)

# =========== Make the dictionary (array of char) ========== #
def makeDict(opt) :

    dictionary = dict()
    available_char = string.ascii_letters + string.digits + string.punctuation + string.whitespace
    i = 0
    if (opt == "compress") :
        for ch in available_char :
            dictionary[ch] = i
            i += 1
    else :
        for ch in available_char :
            dictionary[i] = ch  
            i += 1
    
    return dictionary,i

# ========== Function to Compress ========= #
def doCompress(filename) :

    # Make the dictionary using makeDict() function
    dictionary,i = makeDict("compress")

    # Convert from file to string
    data = readFile(filename)

    # Start the compression
    word = ""
    res = ""
    for c in data :
        wc = word + c
        if wc in dictionary :
            word = wc
        else :
            res += str(dictionary[word]) + "-"
            dictionary[wc] = i
            i += 1
            word = c
    
    if word :
        res += str(dictionary[word])

    writeCompressFile(res,filename)
    
# ========== Function to deCompress ========== #
def doDecompress(filename) :

    # Make the dictionary using makeDict() function
    dictionary,i = makeDict("decompress")

    # Convert from file to string
    data = readFile(filename)
    filetype = data.split(" ")[0]
    data = data.split(" ")[2]
    tempdata = [int(c) for c in data.split("-")]

    # Start the Decompression
    res = StringIO()
    word = dictionary[tempdata[0]]
    res.write(word)
    
    for k in tempdata[1:] :
        if k in dictionary :
            entry = dictionary[k]
        elif k == i :
            entry = word + word[0]

        res.write(entry)

        dictionary[i] = word + entry[0]
        i += 1

        word = entry

    writeDecompressFile(res.getvalue(),filename,filetype)

# ========== Main Function ========== #
def main() :

    # Validation of arguments input
    if (len(sys.argv) == 3) :
        try :
            query = sys.argv[1]
            filename = sys.argv[2].split(".")

            # Validation again
            if (query != "compress" and query != "decompress" and filename[1] != "txt" and filename[1] != "gif" and filename[1] != "tiff" and filename[1] != "tif") :
                print("Wrong arguments ! Check your query and/or your input file :(")
            else :
                if (query == "compress") :
                    doCompress(filename)
                else :
                    doDecompress(filename)
        except(FileNotFoundError) :
            print("File not found :(")
    else :
        print("Wrong size of arguments ! ")
        print("How to use is available in readme :D :D")

# ========== Driver ========== #
if __name__ == "__main__":
    main()