target remote :4242
#monitor arm semihosting enable
# if using ITM
# monitor tpiu config internal itm.fifo uart off 8000000
# monitor itm port 0 on
#load
#tbreak cortex_m_rt::reset_handler
#monitor reset halt
#continue
