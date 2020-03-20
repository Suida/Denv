set runtimepath^=~/.vim runtimepath+=~/.vim/after
let &packpath = &runtimepath
source ~/.vimrc

" Terminal settings

" Exit tmode
tnoremap <Esc> <C-\><C-n>
tnoremap jk <C-\><C-n>

" Ctrl+R
tnoremap <expr> <C-R> '<C-\><C-N>"'.nr2char(getchar()).'pi'

" Navigate
tnoremap <A-h> <C-\><C-N><C-w>h
tnoremap <A-j> <C-\><C-N><C-w>j
tnoremap <A-k> <C-\><C-N><C-w>k
tnoremap <A-l> <C-\><C-N><C-w>l
tnoremap <C-W>h <C-\><C-N><C-w>h
tnoremap <C-W>j <C-\><C-N><C-w>j
tnoremap <C-W>k <C-\><C-N><C-w>k
tnoremap <C-W>l <C-\><C-N><C-w>l
inoremap <A-h> <C-\><C-N><C-w>h
inoremap <A-j> <C-\><C-N><C-w>j
inoremap <A-k> <C-\><C-N><C-w>k
inoremap <A-l> <C-\><C-N><C-w>l
nnoremap <A-h> <C-w>h
nnoremap <A-j> <C-w>j
nnoremap <A-k> <C-w>k
:nnoremap <A-l> <C-w>l

