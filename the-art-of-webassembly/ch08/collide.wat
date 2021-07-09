(module
  (global $cnvs_size (import "env" "cnvs_size") i32)

  (global $no_hit_color (import "env" "no_hit_color") i32)
  (global $hit_color (import "env" "hit_color") i32)
  (global $obj_start (import "env" "obj_start") i32)
  (global $obj_size (import "env" "obj_size") i32)
  (global $obj_cnt (import "env" "obj_cnt") i32)

  (global $x_offset (import "env" "x_offset") i32)
  (global $y_offset (import "env" "y_offset") i32)
  (global $xv_offset (import "env" "xv_offset") i32)
  (global $yv_offset (import "env" "yv_offset") i32)

  (import "env" "buffer" (memory 80))

  ;; clear the entire canvas
  (func $clear_canvas
    (local $i i32)
    (local $pixel_bytes i32)

    global.get $cnvs_size
    global.get $cnvs_size
    i32.mul ;; multiply $width and $height

    i32.const 4
    i32.mul ;; 4 bytes per pixel

    local.set $pixel_bytes ;; $pixel_bytes = $width * $height * 4

    (loop $pixel_loop
      (i32.store (local.get $i) (i32.const 0xff_00_00_00))

      (i32.add (local.get $i) (i32.const 4))
      local.set $i ;; $i += 4 (bytes per pixel)

      ;; if $i < $pixel_bytes
      (i32.lt_u (local.get $i) (local.get $pixel_bytes))
      br_if $pixel_loop ;; break loop if all pixels set
    )
  )

  ;; this function returns an absolute value when a value is passed in
  (func $abs
    (param $value i32)
    (result i32)

    (i32.lt_s (local.get $value) (i32.const 0)) ;; is $value negative?
    if ;; if $value is negative subtract it from 0 to get the positive value
      i32.const 0
      local.get $value
      i32.sub
      return
    end
    local.get $value ;; return original value
  )

  ;; this function sets a pixel at coordinates $x, $y to the color $c
  (func $set_pixel
    (param $x i32) ;; x coordinate
    (param $y i32) ;; y coordinate
    (param $c i32) ;; color value

    ;; is $x > $cnvs_sizes
    (i32.ge_u (local.get $x) (global.get $cnvs_size))
    if ;; $x is outside the canvas bounds
      return
    end

    ;; is $y > $cnvs_sizes
    (i32.ge_u (local.get $y) (global.get $cnvs_size))
    if ;; $y is outside the canvas bounds
      return
    end

    local.get $y
    global.get $cnvs_size
    i32.mul

    local.get $x
    i32.add ;; $x + $y * $cnvs_size (get pixels into linear memory)

    i32.const 4
    i32.mul ;; multiply by 4 because each pixel is 4 bytes

    local.get $c ;; load color value

    i32.store ;; store color in memory location
  )
)
