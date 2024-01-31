import os
import shutil
import datetime as dt

#GB 30/.1/2024
#check if the last backup file is older than a month and in this case 
#create a new backupFile

class fileNameDate:
    def __init__(self, filePath, dateTime) -> None:
        self.filePath = filePath
        self.dateTime = dateTime
    def __gt__(self, other) -> bool:   #compare dateTime
        if not isinstance(other, fileNameDate) :
            return NotImplemented    
        return self.dateTime > other.dateTime
    def __str__(self) -> str:
        return self.filePath + "  " + self.dateTime.__str__()

def copyFile():
    src = ".\\" + filenameToBackup + extFileToBackup
    now = dt.datetime.now()
    filename = filenameToBackup + "_" + str(now.year) + "_" + str(now.month).zfill(2) + "_" + str(now.day).zfill(2) + extFileToBackup
    dest = os.path.join(backupDir, filename )

    print("Sto copiando ", src , " in " , dest)
    shutil.copy2(src, dest)
    print("---Copia finita")

backupDir = ".\\backupDir"
filenameToBackup = "filename"
extFileToBackup = ".ext"


mostRecentFile = fileNameDate("placeholder", dt.datetime.fromtimestamp(0))
#itero sui file e cerco il più recente
# iterate in filename and searching most recent
for filename in os.listdir(backupDir):
    #check extension
    if (filename.endswith(extFileToBackup)):
        filePath = os.path.join(backupDir, filename)
        #store information in a fileNameDate object
        file = fileNameDate(filePath, dt.datetime.fromtimestamp(os.path.getctime(filePath)))
        mostRecentFile = max(mostRecentFile, file)


#check date
if ( mostRecentFile.dateTime < dt.datetime.now() - dt.timedelta(days=30) ):
    copyFile()
else: 
    print("--------")
    print("Il file è già recente")
    print("--------")

input("Premi Enter per terminare...")