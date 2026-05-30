from typing import List
import random

class MasterMind :
    def __init__(self, number_of_digits = 4, allowed_play=20) : 
        self.secret = self.generate_secret(number_of_digits)
        self.play_number = 0
        self.allowed_number = allowed_play
        self.win =  False
    
    def generate_secret(self, n: int = 4) -> List[int] :
        secret = []
        num = random.randint(0,9)
        secret.append(num)
        while len(secret) < n :
            num = random.randint(0,9)
            if not num in secret :
                secret.append(num)
        return secret

    def compare(self,input_number ) :
        input_list = [int(num) for num in list(input_number)]
        green=0
        yellow=0
        red=0
        for i in range(0,4): 
            if input_list[i] == self.secret[i]:
                green +=1
        
        for i in range(0,4): 
            if not input_list[i] in self.secret:
                red +=1
        
        for i in range(0,4): 
            if input_list[i] != self.secret[i] and input_list[i] in self.secret:
                yellow +=1

        self.play_number +=1
        if green ==4 : 
            self.win = True

        return green, yellow, red



if __name__ == '__main__' :
    while True:     
        print("="*50)
        print("          Master Mind - Think & Guess")
        print("="*50)
        game = MasterMind(allowed_play=5)
        # print(f"======== Secret: {game.secret} ===========")
        while game.win != True and game.play_number < game.allowed_number: 
            number = input("Enter your Guess : ")
            g, y, r = game.compare(number)
            print(f"🟢: {g}   🟡: {y}   🔴: {r}")
            print(f"You Have {game.allowed_number-game.play_number} Attemp Left")

        if  game.win :
            print("Hooray! You Cracked the Code!")
        else :
            print("Try Harder!!! - You Lose!")
        r= input("Do You Want To play Another Game? (y/n) [y]")
        if r!='y' and r!='Y' : 
            break

