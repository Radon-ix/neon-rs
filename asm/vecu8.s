.text

.global _add_vecu8_16
.type _add_vecu8_16, %function

._add_vecu8_16:
    ldr q0, [x0]
    ldr q1, [x1]

    add v2.16b, v0.16b, v1.16b
    str q2, [x2]


.global _add_vecu8_8
.type _add_vecu8_8, %function

._add_vecu8_8:
    ldr d0, [x0]
    ldr d1, [x1]

    add v2.8b, v0.8b, v1.8b
    str d2, [x2]