# Learning Operating Systems by building an OS in Rust

[Following along here](https://os.phil-opp.com/)

Latest Commits:

commit 3667c9e


    alloc_from_region function to test and provide valid memory
    
    For context, the find_region function traverses the linked list of
    memory regions in search of a valid free region to use for
    allocation. On each traversal, the logic for testing the validity
    of a memory region is encapsulated within the alloc_from_region
    function which we now define.
    
    The alloc_from_region function uses the start and end address
    to check for an overflow or if the end address is behind the end
    address of the region. In either case, an error is returned to the
    find_region function, which tells it to continue traversing for
    other memory regions.
    
    If neither occurs, the region is still potentially valid. The next
    check performed by alloc_from_region ensures that the remaining
    region is usable after allocation. To do so, the function checks
    that the remainder is either the same size or larger than a
    ListNode.

commit 052ed59


    Create find region method for Linked List Allocator
    
    Implement the find_region method for the LinkedListAllocator struct,
    to search the linked list for a free region suitable to use for a
    particular heap allocation.
    
    We search the linked list iteratively, storing the current node in
    a current variable and iterating with a while let construct that
    loops as long as there are existing regions in the linked list.
    
    On each iteration we check if the current region is suitable in
    terms of both size and alignment with the alloc_from_region
    method which has yet to be implemented.
    
    If it is suitable, it is removed from the linked list by creating
    temp variables and linking the current region's next pointer
    (the current region is the region to be allocated) and assigning
    it to the region after the region to be allocated. With the
    linked list now rebuilt excluding the region to be allocated,
    we return it in a tuple with its aligned alloc_start address.
    
    If the current region is not suitable, we continue searching
    the linked list, using a standard iterative technique of
    reassigning the current variable to the current node's
    next field.

commit b5bc8ff


    Create function to add free memory region to front of linked list.
    
    The add_free_region method implemented on the LinkedListAllocator
    struct is how we push free regions onto the linked list. The method
    takes an address and size and adds that memory region to the front.
    
    We first assert that the region is large enough to store the ListNode,
    then creates a new node with the size of the free region. Option::take
    sets this new node's next pointer to the current head. The node is then
    written to the beginning of the free region and the head is pointed
    to it.

commit 1ae9915


    ChangeLog

commit e9ae2aa


    Create LinkedListAllocator struct using ListNode
    
    The LinkedListAllocator has one field — the head, a ListNode
    pointing to the first region of freed heap memory.
    LinkedListAllocator's new function doesn't initialize the allocator
    with heap bounds because the ALLOCATOR static needs it at compile
    time, whereas writing a node to heap memory must be done at runtime.
    Thus we separate instantiation and initialization into a const fn
    and unsafe fn respectively.
