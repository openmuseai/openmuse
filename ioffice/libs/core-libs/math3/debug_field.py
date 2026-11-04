line = 'use crate::org::apache::commons::math3::Field;'
trimmed = line.strip()
print('endswith Field;:', trimmed.endswith('Field;'))
result = line.replace('::Field;', '::Field::Field;')
print('Result:', result)
print('Count of ::Field; in original:', line.count('::Field;'))
