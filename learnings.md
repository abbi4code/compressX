
//BufReader will wrap our reader(File) with a buffer to imp performance and instead of reading small chunks of data from underlying reader it read larger chunk of data into memory to reduce system calls
->copy function will used to copy all data from one reader (like buffer or file ) to a writer(another file)

->A buffer is a temp storage area in memory that hold data while transfering it between two places 
like disk to memory (reading file)
memory to disk (writing a file)

-> what happens in buffer is data is read/written in larger chunks, and smaller portions are served from the buffer in memory.

