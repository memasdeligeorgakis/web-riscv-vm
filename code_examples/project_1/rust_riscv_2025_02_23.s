


000110f4 <_start>:
   110f4:	1141                	addi	sp,sp,-16
   110f6:	c606                	sw	ra,12(sp)
   110f8:	00000097          	auipc	ra,0x0
   110fc:	018080e7          	jalr	24(ra) # 11110 <main>
   11100:	00012537          	lui	a0,0x12
   11104:	4585                	li	a1,1
   11106:	14b52023          	sw	a1,320(a0) # 12140 <_ZN10rust_riscv6RESULT17ha26bc3dfb5ff0c2cE>
   1110a:	40b2                	lw	ra,12(sp)
   1110c:	0141                	addi	sp,sp,16
   1110e:	8082                	ret

00011110 <main>:
   11110:	1141                	addi	sp,sp,-16
   11112:	c606                	sw	ra,12(sp)
   11114:	4505                	li	a0,1
   11116:	4589                	li	a1,2
   11118:	00000097          	auipc	ra,0x0
   1111c:	018080e7          	jalr	24(ra) # 11130 <sum_2_number>
   11120:	050d                	addi	a0,a0,3
   11122:	000125b7          	lui	a1,0x12
   11126:	14a5a223          	sw	a0,324(a1) # 12144 <_ZN10rust_riscv8RESULT_217hd55270e3bb8c12abE>
   1112a:	40b2                	lw	ra,12(sp)
   1112c:	0141                	addi	sp,sp,16
   1112e:	8082                	ret

00011130 <sum_2_number>:
   11130:	46a9                	li	a3,10
   11132:	463d                	li	a2,15
   11134:	00d56363          	    bltu	a0,a3,1113a <sum_2_number+0xa>
   11138:	4665                	li	a2,25
   1113a:	952e                	add	a0,a0,a1
   1113c:	9532                	add	a0,a0,a2
   1113e:	8082                	ret
