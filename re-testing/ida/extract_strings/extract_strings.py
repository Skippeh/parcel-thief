from idautils import *
from idc import *
from ida_bytes import *
from idaapi import *


# Does some hacky shit to check if bytes are not something other than text
# Returns true if it's considered to be valid text
def do_hacky_checks_on_bytes(bytes):
    return bytes != b"H" and bytes != b"" and bytes != b"AWAVAUATVWUSH"


# Does some hacky shit to check if a string is not something other than text
# Returns true if it's considered to be valid text
def do_hacky_checks_on_string(str):
    return not str.startswith("@")


def extract_strings(cur_addr, end_addr):
    with open("strings.txt", "w") as file:
        last_addr = 0
        first_write = True
        while cur_addr < end_addr:
            if cur_addr == idc.BADADDR:
                break

            deref = ida_bytes.get_qword(cur_addr)

            if deref != None:
                try:
                    bytes = get_strlit_contents(deref, -1, STRTYPE_C)

                    if bytes != None and do_hacky_checks_on_bytes(bytes):
                        str = bytes.decode("ascii")

                        if str != None and do_hacky_checks_on_string(str):
                            new_line = not first_write and cur_addr - last_addr != 8
                            str = "{},{}".format(str, hex(cur_addr))

                            file.write(("\n" if new_line else "") + str + "\n")
                            first_write = False
                            last_addr = cur_addr
                except UnicodeDecodeError:
                    pass

            cur_addr = idc.next_head(cur_addr)


if __name__ == "__main__":
    start = 0x7FF6295FE000
    end = 0x7FF62A002498
    extract_strings(start, end)
