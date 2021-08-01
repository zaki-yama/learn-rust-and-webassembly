(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32 i32)))
  (import "env" "cnvs_size" (global (;0;) i32))
  (import "env" "no_hit_color" (global (;1;) i32))
  (import "env" "hit_color" (global (;2;) i32))
  (import "env" "obj_start" (global (;3;) i32))
  (import "env" "obj_size" (global (;4;) i32))
  (import "env" "obj_cnt" (global (;5;) i32))
  (import "env" "x_offset" (global (;6;) i32))
  (import "env" "y_offset" (global (;7;) i32))
  (import "env" "xv_offset" (global (;8;) i32))
  (import "env" "yv_offset" (global (;9;) i32))
  (import "env" "buffer" (memory (;0;) 80))
  (func (;0;) (type 1) (param i32 i32 i32)
    (local i32 i32 i32)
    (local.set 4
      (i32.add
        (local.tee 3
          (local.get 0))
        (global.get 4)))
    (local.set 5
      (i32.add
        (global.get 4)
        (local.get 1)))
    (loop  ;; label = @1
      (block  ;; label = @2
        (block  ;; label = @3
          (br_if 0 (;@3;)
            (i32.ge_u
              (local.get 3)
              (global.get 0)))
          (br_if 0 (;@3;)
            (i32.ge_u
              (local.get 1)
              (global.get 0)))
          (i32.store
            (i32.shl
              (i32.add
                (i32.mul
                  (local.get 1)
                  (global.get 0))
                (local.get 3))
              (i32.const 2))
            (local.get 2)))
        (if  ;; label = @3
          (i32.ge_u
            (local.tee 3
              (i32.add
                (local.get 3)
                (i32.const 1)))
            (local.get 4))
          (then
            (local.set 3
              (local.get 0))
            (br_if 1 (;@2;)
              (i32.ge_u
                (local.tee 1
                  (i32.add
                    (local.get 1)
                    (i32.const 1)))
                (local.get 5)))))
        (br 1 (;@1;)))))
  (func (;1;) (type 0)
    (local i32 i32 i32 i32 i32 i32)
    (local.set 2
      (i32.shl
        (i32.mul
          (global.get 0)
          (global.get 0))
        (i32.const 2)))
    (loop  ;; label = @1
      (i32.store
        (local.get 0)
        (i32.const -16777216))
      (br_if 0 (;@1;)
        (i32.gt_u
          (local.get 2)
          (local.tee 0
            (i32.add
              (local.get 0)
              (i32.const 4))))))
    (loop  ;; label = @1
      (local.set 0
        (i32.and
          (i32.add
            (i32.load
              (i32.add
                (global.get 9)
                (i32.add
                  (global.get 3)
                  (i32.shl
                    (local.get 1)
                    (i32.const 4)))))
            (i32.load
              (i32.add
                (global.get 7)
                (i32.add
                  (global.get 3)
                  (i32.shl
                    (local.get 1)
                    (i32.const 4))))))
          (i32.const 511)))
      (i32.store
        (i32.add
          (global.get 6)
          (i32.add
            (global.get 3)
            (i32.shl
              (local.get 1)
              (i32.const 4))))
        (i32.and
          (i32.add
            (i32.load
              (i32.add
                (global.get 6)
                (i32.add
                  (global.get 3)
                  (i32.shl
                    (local.get 1)
                    (i32.const 4)))))
            (i32.load
              (i32.add
                (global.get 8)
                (i32.add
                  (global.get 3)
                  (i32.shl
                    (local.get 1)
                    (i32.const 4))))))
          (i32.const 511)))
      (i32.store
        (i32.add
          (global.get 7)
          (i32.add
            (global.get 3)
            (i32.shl
              (local.get 1)
              (i32.const 4))))
        (local.get 0))
      (br_if 0 (;@1;)
        (i32.lt_u
          (local.tee 1
            (i32.add
              (local.get 1)
              (i32.const 1)))
          (global.get 5))))
    (local.set 1
      (i32.const 0))
    (loop  ;; label = @1
      (local.set 5
        (local.tee 0
          (i32.const 0)))
      (local.set 2
        (i32.load
          (i32.add
            (global.get 6)
            (i32.add
              (global.get 3)
              (i32.shl
                (local.get 1)
                (i32.const 4))))))
      (local.set 4
        (i32.load
          (i32.add
            (global.get 7)
            (i32.add
              (global.get 3)
              (i32.shl
                (local.get 1)
                (i32.const 4))))))
      (if  ;; label = @2
        (loop (result i32)  ;; label = @3
          (if (result i32)  ;; label = @4
            (i32.lt_u
              (local.tee 0
                (select
                  (i32.add
                    (local.get 0)
                    (i32.const 1))
                  (local.get 0)
                  (i32.eq
                    (local.get 0)
                    (local.get 1))))
              (global.get 5))
            (then
              (if  ;; label = @5
                (i32.ge_u
                  (block (result i32)  ;; label = @6
                    (drop
                      (br_if 0 (;@6;)
                        (i32.sub
                          (i32.const 0)
                          (local.tee 3
                            (i32.sub
                              (local.get 2)
                              (i32.load
                                (i32.add
                                  (global.get 6)
                                  (i32.add
                                    (global.get 3)
                                    (i32.shl
                                      (local.get 0)
                                      (i32.const 4))))))))
                        (i32.lt_s
                          (local.get 3)
                          (i32.const 0))))
                    (local.get 3))
                  (global.get 4))
                (then
                  (local.set 0
                    (i32.add
                      (local.get 0)
                      (i32.const 1)))
                  (br 2 (;@3;))))
              (if  ;; label = @5
                (i32.ge_u
                  (block (result i32)  ;; label = @6
                    (drop
                      (br_if 0 (;@6;)
                        (i32.sub
                          (i32.const 0)
                          (local.tee 3
                            (i32.sub
                              (local.get 4)
                              (i32.load
                                (i32.add
                                  (global.get 7)
                                  (i32.add
                                    (global.get 3)
                                    (i32.shl
                                      (local.get 0)
                                      (i32.const 4))))))))
                        (i32.lt_s
                          (local.get 3)
                          (i32.const 0))))
                    (local.get 3))
                  (global.get 4))
                (then
                  (local.set 0
                    (i32.add
                      (local.get 0)
                      (i32.const 1)))
                  (br 2 (;@3;))))
              (i32.const 1))
            (else
              (i32.const 0))))
        (then
          (call 0
            (local.get 2)
            (local.get 4)
            (global.get 2)))
        (else
          (call 0
            (local.get 2)
            (local.get 4)
            (global.get 1))))
      (br_if 0 (;@1;)
        (i32.lt_u
          (local.tee 1
            (i32.add
              (local.get 1)
              (i32.const 1)))
          (global.get 5)))))
  (export "main" (func 1)))
