from sys import stdout
from mapper.utils import generate_map
from mapper.utils import make_local_copy


def run_mapper(options):
    out = stdout if not options.file else options.file

    if options.depth:
        node_list = generate_map(options.url, not options.travel, options.depth, out)
    else:
        node_list = generate_map(options.url, not options.travel, file=out)

    if options.copy:
        make_local_copy(node_list, options.path)
