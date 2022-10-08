import cv2
import os

pat = input("Enter Video File Path: ")

if not os.path.exists(pat):
    print("Video does not exists\nAbort...")
    exit
vid = cv2.VideoCapture(pat)

try:

    if os.path.exists('data'):
        os.system("rm -rf data && mkdir data")
except OSError:
    print('Error: Creating directory of data')

currentframe = 0

while (True):

    success, frame = vid.read()
    
    if success:
        name = './data/frame' + str(currentframe) + '.jpg'
        
        cv2.imwrite(name, frame)
        os.system(f"feh --bg-scale {name}")
        
        if not currentframe == 0:
            os.system(f"rm -rf ./data/frame{currentframe-1}.jpg")
        currentframe += 1
    else:
        vid = cv2.VideoCapture(pat)

vid.release()
cv2.destroyAllWindows()
