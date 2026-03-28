if exists('g:loaded_birta')
  finish
endif
let g:loaded_birta = 1

command! BirtaPreview lua require('birta').preview()
command! BirtaStop    lua require('birta').stop()
command! BirtaToggle  lua require('birta').toggle()
