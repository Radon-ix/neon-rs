.text

.global _w_add_vecu8_16
.type _w_add_vecu8_16, %function

_w_add_vecu8_16:
    ld1 {v0.16b}, [x0]
    ld1 {v1.16b}, [x1]

    add v2.16b, v0.16b, v1.16b
    st1 {v2.16b}, [x2]
    ret


.global _w_add_vecu8_8
.type _w_add_vecu8_8, %function

_w_add_vecu8_8:
    ld1 {v0.8b}, [x0]
    ld1 {v1.8b}, [x1]

    add v2.8b, v0.8b, v1.8b
    st1 {v2.8b}, [x2]
    ret


.global _w_sub_vecu8_16
.type _w_sub_vecu8_16, %function

_w_sub_vecu8_16:
    ld1 {v0.16b}, [x0]
    ld1 {v1.16b}, [x1]

    sub v2.16b, v0.16b, v1.16b
    st1 {v2.16b}, [x2]
    ret


.global _w_sub_vecu8_8
.type _w_sub_vecu8_8, %function

_w_sub_vecu8_8:
    ld1 {v0.8b}, [x0]
    ld1 {v1.8b}, [x1]

    sub v2.8b, v0.8b, v1.8b
    st1 {v2.8b}, [x2]
    ret