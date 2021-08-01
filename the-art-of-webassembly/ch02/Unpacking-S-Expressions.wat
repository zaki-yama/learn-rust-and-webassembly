(i32.mul          ;; executes 7th (last)
  (i32.add        ;; executes 3rd
    (i32.const 3) ;; executes 1st
    (i32.const 2) ;; executes 2nd
  )
  (i32.sub        ;; executes 6th
    (i32.const 9) ;; executes 4th
    (i32.const 7) ;; executes 5th
  )
)

(;
(i32.add        ;; executes 3rd
  (i32.const 3) ;; executes 1st
  (i32.const 2) ;; executes 2nd
)
は
i32.const 3
i32.const 2
i32.add
と等価
;)
i32.const 3 ;; Stack = [3]
i32.const 2 ;; Stack = [2, 3]
i32.add ;; 2 & 3 popped from stack, added sum of 5 pushed onto stack [5]

i32.const 9 ;; Stack = [9,5]
i32.const 7 ;; Stack = [7,9,5]
i32.sub ;; 7 & 9 popped off stack . 9-7=2 pushed on stack [2,5]
i32.mul ;; 2,5 popped off stack, 2x5=10 is pushed on the stack [10]
