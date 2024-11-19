import pyexiv2
import os

#   GB 28/01/2024
# divide i file della cartella in cui è lanciato che hanno una determinata estensione in sottodirectory
# in base a data di scatto exif

# split files in the directory where is launched in subdirectories based on EXIF dateTime

path = "."
extensions = [".JPG", ".NEF", ".jpg", ".nef"]

def checkExtension(filename) :
    for ext in extensions : 
        if filename.endswith(ext):
            return True
    return False

def getDateyyyy_mm_dd(dateTime) :
    return dateTime[0:4] + '_' + dateTime[5:7] + '_' + dateTime[8:10]

print( "Script per dividere file in base a data di scatto exif" )
input("Premi Enter per continuare...")

#itero su elementi directory
#iterate on directory element
for filename in os.listdir(path):
    
    #controllo l'estensione
    if checkExtension(filename):
        
        # ex: srcPath = .\filename.ext
        srcPath = os.path.join(path, filename)

        #get data from exif
        with pyexiv2.Image(srcPath) as rawImage:
            exifData = rawImage.read_exif()
        
        #operazioni sulla data
        #format data
        dateTime = exifData['Exif.Image.DateTime']
        date = getDateyyyy_mm_dd(dateTime)

        #costruisco path di destinazione
        #build destination path
        destPath = os.path.join(path, date, filename)
        
        #make the dir if there isn't one with that name
        dirPath = os.path.dirname(destPath)
        os.makedirs(dirPath, exist_ok=True)

        #copio effetivamente i file
        #copy the file
        os.rename(srcPath, destPath)
        print("sto copiando da:", srcPath, " a:", destPath)

print("Finito")
input("Premi Enter per uscire...")