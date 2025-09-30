# Known Answer Tests (KATs) README

The files follow naming convention
- T for triple des
- The mode (use EBC for simplicity)
- name of what the test is
 - invperm tests the inversion permutation
 - permop tests the permuation operation
 - subtab tests the S blocks
 - varkey varies the key and keeps the plaintext constant
 - vartext varies the plaintext and keeps the key constant

For now, the ideal would be using ECB mode to avoid figuring out the IV
Ideally, should write some handler to pass each test automatically.

