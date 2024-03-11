= PDF addition
Say we have the following PDFs:
$ A = mat(1, 2; 0.2, 0.8); B = mat(2, 3; 0.4, 0.6) $
And we want to find the PDF $C = A + B$.
$ C = mat(3, 4, 5; (0.2 dot 0.4), (0.2 dot 0.6 + 0.8 dot 0.4), (0.8 dot 0.6)) = mat(3, 4, 5; 0.08, 0.44, 0.48) $
Implementing fast convolution algorithms for this is gonna be a little painful, and will be left for later. For now, this library will focus on correctness, ergonomics, and generality.