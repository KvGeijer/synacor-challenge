import subprocess
import random
import re

initial_path = subprocess.run(["cat", "tunnel-path.in"], capture_output=True)
initial_path = initial_path.stdout

def test_path(twisty_path):
	path = initial_path + twisty_path
	cargo = subprocess.run(["cargo", "run"], input=path, capture_output=True)

	output = cargo.stdout.decode('utf-8')
	# print(output)
	place, descr = output.split("==")[-2:]
	return place, descr

def print_final(twisty_path):
	place, descr = test_path(twisty_path)
	print(f"=={place}=={descr}")

def test_coords(north, east):
	path = b"north\n"*north + b"east\n"*east + b"south\n"*-north + b"west\n"*-east
	test_path(path)

def get_room(descr):
	return descr.split("\n")[1]

def get_options(descr):
	return [dir[2:] for dir in re.findall(r'- \w+\n', descr)]

# Abandoned
def brute():
	noninteresting = [
		"You are in a twisty maze of little passages, all alike.",
		"You are in a maze of alike little passages, all twisty.",
		"You are in a maze of little twisty passages, all alike.",
		# "You are in a maze of twisty little passages, all dimly lit by more bioluminescent moss.  There is a ladder here leading up."
	]
	
	path = b""
	place, descr = test_path(path)
	while get_room(descr) == "You are in a maze of twisty little passages, all dimly lit by more bioluminescent moss.  There is a ladder here leading up.":
		path = b""
		place, descr = test_path(path)

		while get_room(descr) in noninteresting:
			options = get_options(descr)
			path = path + bytes(options[random.randint(0, len(options)-1)], 'utf-8')
			place, descr = test_path(path)
		print("Back to start")
	print(get_room(descr))
	print(f"Finished brute force with {path.decode('utf-8')}\n== {place} ==\n{descr}")

def dfs(path, visited):
	place, descr = test_path(path)
	print("===============")
	print(path)
	print(descr)
	print("===============")
	if descr not in visited:
		visited.append(descr)
		cont = False
		for option in get_options(descr):
			cont |= dfs(path + bytes(option, 'utf-8'), visited)
		if not cont:
			print(f"==================\nDeadend :D\nPath: {path}\n=={place}==\n{descr}\n========================")
		visited.pop()
		return True
	else:
		return False

def maze_dfs():
	dfs(b"", [])

maze_dfs()
# brute()

# print_final(
# b"""south
# """)

# test_coords(5, -1)
