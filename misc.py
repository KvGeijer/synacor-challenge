import itertools

def monument(args):
	return args[0] + args[1]*args[2]**2+args[3]**3 - args[4] == 399

coins = {
		"red": 2,
		"blue": 9,
		"corroded": 3,
		"concave": 7,
		"shiny": 5,
	}

def find_coins():
	for perm in itertools.permutations(coins.keys()):
		if monument([coins[coin] for coin in perm]):
			print("Found a match!")
			for coin in perm:
				print(f"use {coin} coin")


maze = [
	['*', 8, '-', 1],
	[4, '*', 11, '*'],
	['+', 4, '-', 18],
	[22, '-', 9, '*'],
]

def neighs(pos):
	cand = [pos + off for off in [1, -1, 1j, -1j]]
	return [ind for ind in cand if (0 <= ind.real <= 3) and (0 <= ind.imag <= 3)]

def getat(pos):
	return maze[round(pos.real)][round(pos.imag)]

def maze30():
	reach = [(3 + 0j, 22, None, [])]
	for rounds in range(1, 100):
		print(rounds)
		next = []
		for (pos, carry, op, path) in reach:
			# Take one step
			for neigh in neighs(pos):
				npath = path + [neigh]
				nop = op
				ncarry = carry
				match getat(neigh):
					case '+':
						nop = lambda x, y: x + y  
					case '*':
						nop = lambda x, y: x * y  
					case '-':
						nop = lambda x, y: x - y  
					case num:
						ncarry = op(carry, num)
				if neigh == 3:
					pass
				elif neigh == 3j:
					if carry == 30:
						print(f"Found a path in {rounds}: {npath}")
						return
				else:
					# Add it as a possible next state
					next.append((neigh, ncarry, nop, npath))
		reach = next
				
maze30()


# Chill day, so transcribed a bunch of code. But really only seems to be important arount 6050
INTWRAP = 32768
def recreate():
	# Seems to start at pc 5489 -> 6027, regs: [4, 1, 3, 10, 101, 0, 0, ?]
	r8 = 1
	regs = [4, 1, 3, 10, 101, 0, 0, r8]
	stack = []	# Should it start with more stuff?
	call6027(regs, stack)
	# The rest of this was probably not needed... Good way to waste hungover time
	regs[1] = (regs[0] + 6) % INTWRAP
	if regs[1] == 0:
		# Jump 5579
		stack.push(regs[0])
		stack.push(regs[1])
		stack.push(regs[2])
		regs[0] = 29400
		regs[1] = 1531
		regs[2] = (10724 + 11534) % INTWRAP
		call1458(regs, stack)
		regs[2] = stack.pop()
		regs[1] = stack.pop()
		regs[0] = stack.pop()
	else:
		stack.push(regs[0])
		stack.push(regs[1])
		stack.push(regs[2])
		regs[0] = 29014
		regs[1] = 1531
		regs[2] = (13005 + 4527) % INTWRAP
		call1458(regs, stack)
		regs[2] = stack.pop()
		regs[1] = stack.pop()
		regs[0] = stack.pop()
		regs[0] = regs[7]
		regs[1] = 25866
		regs[2] = 32767
		stack.push(regs[3])
		regs[3] = 29241
		call1841(regs, stack)
		regs[2] = stack.pop()
		stack.push(regs[0])
		stack.push(regs[1])
		stack.push(regs[2])
		regs[0] = 29245
		regs[1] = 1531
		regs[2] = (14852 + 11374) % INTWRAP
		call1458(regs, stack)
		regs[2] = stack.pop()
		regs[1] = stack.pop()
		regs[0] = stack.pop()
		memory[2732] = 2498
		memory[2733] = 0
		regs[1] = 2710
		memory[2710] = 32767
	# Jump 5714
	regs[2] = stack.pop()
	regs[1] = stack.pop()
	regs[0] = stack.pop()
	return

def recreate_opt():
	# Seems to start at pc 5489 -> 6027, regs: [4, 1, 3, 10, 101, 0, 0, ?]
	r8 = 1
	regs = [4, 1, 3, 10, 101, 0, 0, r8]
	stack = []	# Should it start with more stuff?
	call6027(regs, stack)
	# The rest of this was probably not needed... Good way to waste hungover time
	regs[1] = (regs[0] + 6) % INTWRAP
	if regs[1] == 0:
		# Jump 5579
		stack.push(regs[0])
		stack.push(regs[1])
		stack.push(regs[2])
		regs[0] = 29400
		regs[1] = 1531
		regs[2] = (10724 + 11534) % INTWRAP
		call1458(regs, stack)
		regs[2] = stack.pop()
		regs[1] = stack.pop()
		regs[0] = stack.pop()
	else:
		stack.push(regs[0])
		stack.push(regs[1])
		stack.push(regs[2])
		regs[0] = 29014
		regs[1] = 1531
		regs[2] = (13005 + 4527) % INTWRAP
		call1458(regs, stack)
		regs[2] = stack.pop()
		regs[1] = stack.pop()
		regs[0] = stack.pop()
		regs[0] = regs[7]
		regs[1] = 25866
		regs[2] = 32767
		stack.push(regs[3])
		regs[3] = 29241
		call1841(regs, stack)
		regs[2] = stack.pop()
		stack.push(regs[0])
		stack.push(regs[1])
		stack.push(regs[2])
		regs[0] = 29245
		regs[1] = 1531
		regs[2] = (14852 + 11374) % INTWRAP
		call1458(regs, stack)
		regs[2] = stack.pop()
		regs[1] = stack.pop()
		regs[0] = stack.pop()
		memory[2732] = 2498
		memory[2733] = 0
		regs[1] = 2710
		memory[2710] = 32767
	# Jump 5714
	regs[2] = stack.pop()
	regs[1] = stack.pop()
	regs[0] = stack.pop()
	return

def call1841(regs, stack):
	stack.push(regs[3])
	stack.push(regs[4])
	stack.push(regs[5])
	stack.push(regs[6])
	regs[6] = 1
	# 1852
	while True:
		regs[4] = (regs[3] + regs[6]) % INTWRAP
		regs[4] = memory[regs[4]]
		regs[5] = (6125 + regs[6])
		memory[regs[5]] = regs[4]
		regs[6] = (regs[6] + 1) % INTWRAP
		regs[5] = memory[6125]
		regs[4] = regs[6] > regs[5]
		if regs[4] != 0:
			# Don't jump to 1852
			break
	# 1880
	regs[3] = 0
	regs[4] = 0
	# 1886
	regs[5] = memory[6125]
	regs[5] = regs[4] % regs[5] 
	regs[5] = (regs[5] + 6125) % INTWRAP
	regs[5] = (regs[5] + 1) % INTWRAP
	regs[6] = memory[regs[5]]
	regs[6] = (regs[6] * 5249) % INTWRAP
	regs[6] = (regs[6] + 12345) % INTWRAP
	memory[regs[5]] = regs[6]
	stack.append(regs[0])
	stack.append(regs[1])
	regs[1] = regs[6]
	call2125(regs, memory)
	regs[6] = regs[0]
	regs[1] = stack.pop()
	regs[0] = stack.pop()
	regs[5] = memory[regs[1]]
	regs[6] = regs[6] % regs[5]
	regs[6] = (regs[6 + 1]) % INTWRAP
	regs[5] = regs[5] > 1
	if not regs[5]:
		regs[3] = 1
	regs[6] = (regs[6] + regs[1]) % INTWRAP
	regs[6] = memory[regs[6]]
	regs[4] = (regs[4] + 1) % INTWRAP
	regs[5] = (regs[4] + 6129) % INTWRAP
	memory[regs[5]] = regs[6]
	regs[5] = memory[6129]
	regs[5] = regs[4] == regs[5]
	# So some sort of long loop
	if regs[5]  == 0:
		pass
		# Jump to 1886
		# Like below, but does not reset two regs
	if regs[3] == 0:
		pass
		# Jump to 1880
	stack.append(regs[0])
	regs[0] = 6129
	call1518(regs, stack)
	regs[0] = stack.pop()
	regs[6] = stack.pop()
	regs[5] = stack.pop()
	regs[4] = stack.pop()
	regs[3] = stack.pop()
	return

def call1518(regs, stack):
	stack.append(regs[1])
	regs[1] = 1528
	call1458(regs, stack)
	regs[1] = stack.pop()
	return
	
def call2125(regs, stack):
	stack.push(regs[1])
	stack.push(regs[2])
	regs[2] = regs[0] & regs[1]
	regs[2] = ~regs[2]
	regs[0] = regs[0] | regs[1]
	regs[0] = regs[0] & regs[2]
	regs[2] = stack.pop()
	regs[1] = stack.pop()
	return
	
# Takes forever to execute
def call6027(regs, stack):
	# Seems to start at pc 5489 -> 6027, regs: [4, 1, 3, 10, 101, 0, 0, ?]
	if regs[0] != 0:
		# Jump to 6035
		if regs[1] != 0:
			# Jump to 6048
			stack.append(regs[0]) # Should we go back to where it was called from?
			regs[1] = regs[1] - 1
			call6027()
			regs[1] = regs[0]
			regs[0] = stack.pop()
			regs[0] = regs[0] - 1
			call6027(regs, stack)
			return
		else:
			regs[0] = regs[0] - 1
			regs[1] = regs[7]
			call6027(regs, stack)
			return
	else:
		regs[0] = (regs[1] + 1) % INTWRAP # Become 0 here?
		return
	
def call6027opt(regs, stack):
	while regs[0] != 0:
		if regs[1] != 0:
			stack.append(regs[0])
			regs[1] = regs[1] - 1
		else:
			regs[0] = regs[0] - 1
			regs[1] = regs[7]
	regs[0] = (regs[1] + 1) % INTWRAP

def call1458(regs, stack, memory):
	stack.append(regs[0])
	stack.append(regs[3])
	stack.append(regs[4])
	stack.append(regs[5])
	stack.append(regs[6])
	def ret():
		regs[6] = stack.pop()
		regs[5] = stack.pop()
		regs[4] = stack.pop()
		regs[3] = stack.pop()
		regs[0] = stack.pop()
	regs[6] = regs[0]
	regs[5] = regs[1]
	regs[4] = memory[regs[0]]	# How to optimize this?
	regs[1] = 0
	regs[3] = (regs[1] + 1) % INTWRAP
	regs[0] = int(regs[3] > regs[4])
	if regs[0] != 0:
		# Jump 1507
		ret()
		return		
	else:
		while True:
			# 1480
			regs[3] = (regs[3] + regs[6]) % INTWRAP
			regs[0] = memory[regs[3]]	# How to optimize this?
			call(regs[5])
			regs[1] = (regs[1] + 1) % INTWRAP
			if regs[1] == 0:
				# Don't jump to 1480
				ret()
				return		
	