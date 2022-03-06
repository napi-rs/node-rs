use blowfish::Blowfish;

fn setup(cost: u32, salt: &[u8], key: &[u8]) -> Blowfish {
  assert!(cost < 32);
  let mut state = Blowfish::bc_init_state();

  state.salted_expand_key(salt, key);
  for _ in 0..1u32 << cost {
    state.bc_expand_key(key);
    state.bc_expand_key(salt);
  }

  state
}

pub fn bcrypt(cost: u32, salt: &[u8], password: &[u8], output: &mut [u8]) {
  assert!(salt.len() == 16);
  assert!(!password.is_empty() && password.len() <= 72);
  assert!(output.len() == 24);

  let state = setup(cost, salt, password);
  // OrpheanBeholderScryDoubt
  #[allow(clippy::unreadable_literal)]
  let mut ctext = [
    0x4f727068, 0x65616e42, 0x65686f6c, 0x64657253, 0x63727944, 0x6f756274,
  ];
  for i in 0..3 {
    let i: usize = i * 2;
    for _ in 0..64 {
      let [l, r] = state.bc_encrypt([ctext[i], ctext[i + 1]]);
      ctext[i] = l;
      ctext[i + 1] = r;
    }

    let buf = ctext[i].to_be_bytes();
    output[i * 4..][..4].copy_from_slice(&buf);
    let buf = ctext[i + 1].to_be_bytes();
    output[(i + 1) * 4..][..4].copy_from_slice(&buf);
  }
}
