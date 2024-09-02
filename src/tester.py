import subprocess
import re


def run_and_test_program(program, args, input_data, regex_list, message):
    """
    Runs a program with the given arguments and input data, and checks if the output
    matches the provided regular expressions.

    Args:
        program (str): The path to the program to run.
        args (list): List of arguments for the program.
        input_data (str): Multi-line input to be fed to the program.
        regex_list (list): List of regex patterns to check in the program output.

    Returns:
        None
    """
    # Start the program and communicate input data
    process = subprocess.Popen(
        [program] + args,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    stdout, stderr = process.communicate(input=input_data)

    # Capture the output
    full_output = stdout + stderr

    # Check each regex pattern in the output
    for regex in regex_list:
        if not re.search(regex, full_output, re.MULTILINE | re.DOTALL):
            print(f"Error: Pattern '{regex}' not found in output:")
            print(f"---\n{full_output}\n---")
        else:
            print(f"{message} /{regex}/: OK")


# Example usage
if __name__ == "__main__":
    program = "cargo"  # replace with actual program name or path
    args = ["run"]
    input_data1 = """i
one
two
three
four
five
.
3,4p
q
"""
    regex_list1 = [r"three\nfour"]

    run_and_test_program(program, args, input_data1, regex_list1, "print from middle")

    input_data2 = """i
one
two
three
four
five
.
3,4n
q
"""
    regex_list2 = [r"3\tthree\n\s+4\tfour"]
    run_and_test_program(
        program, args, input_data2, regex_list2, "print from middle with numbers"
    )
