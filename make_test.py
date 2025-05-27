import os

PROBLEMS = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']
TEMPLATE = '''
#include <gtest/gtest.h>
#include <cstdio>
#include <fstream>
#include <string>
#include <cctype>

void run_solution(const std::string& exe, const std::string& input, std::string& output) {{
    std::string cmd = exe + " < " + input;
    FILE* pipe = popen(cmd.c_str(), "r");
    if (!pipe) throw std::runtime_error("popen failed");
    char buffer[4096];
    output.clear();
    while (fgets(buffer, sizeof(buffer), pipe)) {{
        output += buffer;
    }}
    pclose(pipe);
    output.erase(
        std::find_if(
            output.rbegin(), 
            output.rend(), 
            [](unsigned char ch) {{return !std::isspace(ch);}}).base(), 
            output.end()
        );
}}

{tests}
'''

def escape_cpp_string(s):
    return s.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n')

ROOT_DIR = os.path.abspath(os.path.dirname(__file__))

for problem in PROBLEMS:
    tdir = os.path.join(ROOT_DIR, 'test', problem)
    if not os.path.exists(tdir):
        continue

    inputs = sorted([f for f in os.listdir(tdir) if f.startswith('input')])
    test_cases = []

    for idx, input_file in enumerate(inputs):
        in_path = os.path.abspath(os.path.join(tdir, input_file))
        out_path = in_path.replace('input', 'output')

        if not os.path.exists(out_path):
            continue

        with open(out_path, 'r') as f:
            expected = escape_cpp_string(f.read())

        exe_path = os.path.abspath(os.path.join(ROOT_DIR, 'build', problem, problem))

        test_case = f'''
TEST({problem.upper()}, Case{idx}) {{
    std::string output;
    run_solution("{exe_path}", "{in_path}", output);
    EXPECT_EQ(output, "{expected}");
}}
'''
        test_cases.append(test_case)

    os.makedirs(os.path.join(ROOT_DIR, 'tests'), exist_ok=True)
    out_file = os.path.join(ROOT_DIR, 'tests', f'generated_test_{problem}.cpp')
    with open(out_file, 'w') as f:
        f.write(TEMPLATE.format(tests=''.join(test_cases)))
