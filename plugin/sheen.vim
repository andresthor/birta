if exists('g:loaded_sheen')
  finish
endif
let g:loaded_sheen = 1

command! SheenPreview lua require('sheen').preview()
command! SheenStop    lua require('sheen').stop()
command! SheenToggle  lua require('sheen').toggle()
