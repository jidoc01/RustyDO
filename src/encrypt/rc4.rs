pub fn transfer<T, U> (mut data: T, key: U) where T: AsMut<[u8]>, U: AsRef<[u8]>{
    let mut state = gen_state(key.as_ref());
    transfer_aux(data.as_mut(), &mut state);
}

fn gen_state(key: &[u8]) -> [u8; 256] {
    let mut state = [0u8; 256];
    for i in 0..256 {
        state[i] = i as u8;
    }

    let mut j = 0usize;
    for i in 0..256 {
        j = (j + state[i] as usize + key[i % key.len()] as usize) % 256;
        state.swap(i, j);
    }

    state
}

fn transfer_aux(data: &mut [u8], state: &mut [u8; 256]) {
    let mut i = 0usize;
    let mut j = 0usize;
    for byte in data.iter_mut() {
        i = (i + 1) % 256;
        j = (j + state[i] as usize) % 256;
        state.swap(i, j);
        let k = state[(state[i] as usize + state[j] as usize) % 256];
        *byte ^= k;
    }
}