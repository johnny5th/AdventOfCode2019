import os
input_data = os.path.join(os.path.dirname(__file__), 'input.txt')

instructions = []
with open(input_data) as f:
    instructions = [int(i) for i in f.readline().split(',')]

def level1():
    print(runInstructions(12, 2, instructions[:]))

def level2():
    for noun in range(0, 99):
        for verb in range(0, 99):
            if runInstructions(noun, verb, instructions[:]) == 19690720:
                print(100 * noun + verb)

def runInstructions(noun, verb, instructions):
    instructions[1] = noun
    instructions[2] = verb
    for opCodeLoc in range(0, len(instructions), 4):
        opCode = instructions[opCodeLoc]
        if opCode == 1:
            instructions[instructions[opCodeLoc+3]] = instructions[instructions[opCodeLoc+1]] + instructions[instructions[opCodeLoc+2]]
        elif opCode == 2:
            instructions[instructions[opCodeLoc+3]] = instructions[instructions[opCodeLoc+1]] * instructions[instructions[opCodeLoc+2]]
        elif opCode == 99:
            return instructions[0]

level1()
level2()