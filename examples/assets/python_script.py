import os
import os.path
import sys



# Require Linux
if sys.platform != 'linux':
    print('This script only works on Linux')
    sys.exit(1)


temp_file = sys.argv[1]
if not os.path.isfile(temp_file):
    print('File not found')
    sys.exit(1)

# Write to file
with open(temp_file, 'w') as f:
    f.write('Hello from Python script')