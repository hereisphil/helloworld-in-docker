# Used this video for the datetime
# https://www.youtube.com/watch?v=-AlFiS74aQg
import datetime
print("Hello ASL!")

time = datetime.datetime.now().strftime("%A, %B %d, %Y at %H:%M")
print("Process completed on:", time, "UTC") # within a container this is UTC