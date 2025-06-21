import os
import sys;

PROBLEMS = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I']

if len(sys.argv) > 1:
    PROBLEMS = sys.argv[1:]

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
    tdir = os.path.join(ROOT_DIR, 'test', problem.upper())
    if not os.path.exists(tdir):
        continue

    answers = sorted([f for f in os.listdir(tdir) if f.startswith(problem.upper()) and f.endswith('.ans')])
    test_cases = []

    for idx, answer_file in enumerate(answers):
        ans_path = os.path.abspath(os.path.join(tdir, answer_file))
        input_path = ans_path.replace('.ans', '')

        if not os.path.exists(input_path):
            continue

        with open(ans_path, 'r') as f:
            raw = f.read()
            # 末尾のすべての空白文字（改行含む）を削除
            raw = raw.rstrip()
            expected = escape_cpp_string(raw)

        exe_path = os.path.abspath(os.path.join(ROOT_DIR, 'build', problem.lower(), problem.lower()))

        test_case = f'''
TEST({problem.upper()}, Case{idx}) {{
    std::string output;
    run_solution("{exe_path}", "{input_path}", output);
    EXPECT_EQ(output, "{expected}");
}}
'''
        test_cases.append(test_case)

    os.makedirs(os.path.join(ROOT_DIR, 'tests'), exist_ok=True)
    out_file = os.path.join(ROOT_DIR, 'tests', f'generated_test_{problem.lower()}.cpp')
    with open(out_file, 'w') as f:
        f.write(TEMPLATE.format(tests=''.join(test_cases)))
