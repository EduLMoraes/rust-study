import tkinter as tk
from tkinter import *
from tkinter import Canvas

class Window():
    def __init__(self):
        self.window = Tk()
        self.window.title("Investida")
        self.window.geometry("400x600")
        self.window.configure(background="#333")
        
        self.frame = Frame(self.window)
        self.frame.pack()

        self.addButton("Calcular")
        
        self.window.mainloop()
    
    def addButton(self, text, command = None, width = None, row = None, column = None, background = None):
        self.button = Button(self.frame,
                             text=text,
                             command=command,
                             width=width,
                             font="bold",
                             background=background,
                             fg="white")
        self.button.grid(row=row, column=column)
        
    def addInput(self, text, width = None, row = None, column = None):
        self.entry = Entry(self.window, text = text, width=width)
        self.entry.grid(row=row, column=column)
        
    def addText(self, text = None, textvariable = None, width = None, justify = "right", row = None, column = None):
        self.text = Label(self.window,
                          textvariable=str(self.texto),
                          width=width, font="bold",
                          background="#333",
                          height="5",
                          justify=justify,
                          fg="white")
        self.text.pack()
        
Window()