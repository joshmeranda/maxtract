from mapper.utils import generate_map
from mapper.utils import make_local_copy


def run_mapper(options):
    if options.depth:
        node_list = generate_map(options.url, not options.travel, options.depth)
    else:
        node_list = generate_map(options.url, not options.travel)

    if options.copy:
        make_local_copy(node_list, options.path)

    for node in node_list:
        print(str(node))
