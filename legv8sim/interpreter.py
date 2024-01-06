import legv8sim as lsim
import json

def parse():
    fname = open(input("What file do you want to open and run?: "), "r")
    program = []
    branchline = list(map(lambda x: x[:x.find("//")]+" " if x.find("//")!=-1 else x, fname.read().split("\n")))
    if len(branchline) == 0:
        return
    branches = dict()
    
    branchlocs = [i for i,e in enumerate(branchline) if ':' in e]
    print(branchlocs)
    if len(branchlocs) ==0:
        branches[""]=branchline
    ir = dict()
    for i in range(len(branchlocs)):
        try:
            branches[branchline[branchlocs[i]]] = branchline[branchlocs[i]+2:branchlocs[i+1]]
        except IndexError:
            branches[branchline[branchlocs[i]]] = branchline[branchlocs[i]+2:]
    if len(branchlocs)>0:
        branches[branchline[branchlocs[-1]]] = branchline[branchlocs[-1]+2:]
    print(branches.keys())
    for branch in branches:
        ir[branch] = []
        for line in branches[branch]:
            funcs = [lsim.r_inst, lsim.i_inst, lsim.d_inst, lsim.b_inst, lsim.cb_inst]
            res=""
            for func in funcs:
                try:
                    res = func(line)
                    break
                except:
                    pass
            if res=="" and line!="":
                raise Exception("issue with running instruction: " + line)
            else:
                ir[branch].append(res)
    return ir

def interpret():
    print("interpreting...")
if __name__ == "__main__":
    print(parse())