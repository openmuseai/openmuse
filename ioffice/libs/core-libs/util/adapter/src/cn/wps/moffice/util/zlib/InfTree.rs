// Java field: int MANY
pub(crate) const MANY: i32 = 1440;
// Java field: int fixed_bl
pub(crate) const fixed_bl: i32 = 9;
// Java field: int fixed_bd
pub(crate) const fixed_bd: i32 = 5;
// Java field: int[] fixed_tl
static fixed_tl: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
// Java field: int[] fixed_td
static fixed_td: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
// Java field: int[] cplens
static cplens: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
// Java field: int[] cplext
static cplext: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
// Java field: int[] cpdist
static cpdist: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
// Java field: int[] cpdext
static cpdext: std::sync::OnceLock<Vec<i32>> = std::sync::OnceLock::new();
// Java field: int BMAX
pub(crate) const BMAX: i32 = 15;

// Java: public class InfTree
pub struct InfTree {
    // Java field: int[] hn
    hn: Option<Vec<i32>>,
    // Java field: int[] v
    v: Option<Vec<i32>>,
    // Java field: int[] c
    c: Option<Vec<i32>>,
    // Java field: int[] r
    r: Option<Vec<i32>>,
    // Java field: int[] u
    u: Option<Vec<i32>>,
    // Java field: int[] x
    x: Option<Vec<i32>>,
}

// Java: public class InfTree
impl InfTree {
    // Java: int huft_build(int[] b, int bindex, int n, int s, int[] d, int[] e, int[] t, int[] m, int[] hp, int[] hn, int[] v)
    pub fn huft_build(&mut self, _b: &[i32], _bindex: i32, _n: i32, _s: i32, _d: Option<&[i32]>, _e: Option<&[i32]>, _t: &mut [i32], _m: &mut [i32], _hp: &mut [i32], _hn: &mut [i32], _v: &mut [i32]) -> i32 {
      // Java: int p = 0;
      // Java: int i = n;
      // Java: do
      {
         // Java: int var10002 = this.c[b[bindex + p]]++;
         // Java: ++p;
         // Java: --i;
      }
      // Java: if (this.c[0] == n)
      {
         // Java: t[0] = -1;
         // Java: m[0] = 0;
         // Java: return 0;
      }
      // Java: else
      {
         // Java: int l = m[0];
         // Java: int j;
         // Java: for(j = 1; j <= 15 && this.c[j] == 0; ++j)
         {
         }
         // Java: int k = j;
         // Java: if (l < j)
         {
            // Java: l = j;
         }
         // Java: for(i = 15; i != 0 && this.c[i] == 0; --i)
         {
         }
         // Java: int g = i;
         // Java: if (l > i)
         {
            // Java: l = i;
         }
         // Java: m[0] = l;
         // Java: int y;
         // Java: for(y = 1 << j; j < i; y <<= 1)
         {
            // Java: if ((y = y - this.c[j]) < 0)
            {
               // Java: return -3;
            }
            // Java: ++j;
         }
         // Java: if ((y = y - this.c[i]) < 0)
         {
            // Java: return -3;
         }
         // Java: else
         {
            // Java: int[] var10000 = this.c;
            // Java: var10000[i] += y;
            // Java: j = 0;
            // Java: this.x[1] = 0;
            // Java: p = 1;
            // Java: int xp = 2;
            // Java: while(true)
            {
               // Java: --i;
               // Java: if (i == 0)
               {
                  // Java: i = 0;
                  // Java: p = 0;
                  // Java: do
                  {
                     // Java: if ((j = b[bindex + p]) != 0)
                     {
                        // Java: v[this.x[j]++] = i;
                     }
                     // Java: ++p;
                     // Java: ++i;
                  }
                  // Java: n = this.x[g];
                  // Java: i = 0;
                  // Java: this.x[0] = 0;
                  // Java: p = 0;
                  // Java: int h = -1;
                  // Java: int w = -l;
                  // Java: this.u[0] = 0;
                  // Java: int q = 0;
                  // Java: for(int z = 0; k <= g; ++k)
                  {
                     // Java: int a = this.c[k];
                     // Java: while(a-- != 0)
                     {
                        // Java: while(k > w + l)
                        {
                           // Java: ++h;
                           // Java: w += l;
                           // Java: z = g - w;
                           // Java: z = z > l ? l : z;
                           // Java: int f;
                           // Java: if ((f = 1 << (j = k - w)) > a + 1)
                           {
                              // Java: f -= a + 1;
                              // Java: xp = k;
                              // Java: if (j < z)
                              {
                                 // Java: while(true)
                                 {
                                    // Java: ++j;
                                    // Java: if (j >= z)
                                    {
                                       // Java: break;
                                    }
                                    // Java: int var29;
                                    // Java: int var49 = var29 = f << 1;
                                    // Java: ++xp;
                                    // Java: if (var49 <= this.c[xp])
                                    {
                                       // Java: break;
                                    }
                                    // Java: f = var29 - this.c[xp];
                                 }
                              }
                           }
                           // Java: z = 1 << j;
                           // Java: if (hn[0] + z > 1440)
                           {
                              // Java: return -3;
                           }
                           // Java: this.u[h] = q = hn[0];
                           // Java: hn[0] += z;
                           // Java: if (h != 0)
                           {
                              // Java: this.x[h] = i;
                              // Java: this.r[0] = (byte)j;
                              // Java: this.r[1] = (byte)l;
                              // Java: j = i >>> w - l;
                              // Java: this.r[2] = q - this.u[h - 1] - j;
                              // Java: System.arraycopy(this.r, 0, hp, (this.u[h - 1] + j) * 3, 3);
                           }
                           // Java: else
                           {
                              // Java: t[0] = q;
                           }
                        }
                        // Java: this.r[1] = (byte)(k - w);
                        // Java: if (p >= n)
                        {
                           // Java: this.r[0] = 192;
                        }
                        // Java: else if (v[p] < s)
                        {
                           // Java: this.r[0] = (byte)(v[p] < 256 ? 0 : 96);
                           // Java: this.r[2] = v[p++];
                        }
                        // Java: else
                        {
                           // Java: this.r[0] = (byte)(e[v[p] - s] + 16 + 64);
                           // Java: this.r[2] = d[v[p++] - s];
                        }
                        // Java: int f = 1 << k - w;
                        // Java: for(int var38 = i >>> w; var38 < z; var38 += f)
                        {
                           // Java: System.arraycopy(this.r, 0, hp, (q + var38) * 3, 3);
                        }
                        // Java: for(j = 1 << k - 1; (i & j) != 0; j >>>= 1)
                        {
                           // Java: i ^= j;
                        }
                        // Java: i ^= j;
                        // Java: for(int mask = (1 << w) - 1; (i & mask) != this.x[h]; mask = (1 << w) - 1)
                        {
                           // Java: --h;
                           // Java: w -= l;
                        }
                     }
                  }
                  // Java: return y != 0 && g != 1 ? -5 : 0;
               }
               // Java: this.x[xp] = j += this.c[p];
               // Java: ++xp;
               // Java: ++p;
            }
         }
      }
      todo!()
    }

    // Java: int inflate_trees_bits(int[] c, int[] bb, int[] tb, int[] hp, ZStream z)
    pub fn inflate_trees_bits(&mut self, _c: &mut [i32], _bb: &mut [i32], _tb: &mut [i32], _hp: &mut [i32], _z: &mut dyn std::any::Any) -> i32 {
      // Java: this.initWorkArea(19);
      // Java: this.hn[0] = 0;
      // Java: int result = this.huft_build(c, 0, 19, 19, (int[])null, (int[])null, tb, bb, hp, this.hn, this.v);
      // Java: if (result == -3)
      {
         // Java: z.msg = "oversubscribed dynamic bit lengths tree";
      }
      // Java: else if (result == -5 || bb[0] == 0)
      {
         // Java: z.msg = "incomplete dynamic bit lengths tree";
         // Java: result = -3;
      }
      // Java: return result;
      todo!()
    }

    // Java: int inflate_trees_dynamic(int nl, int nd, int[] c, int[] bl, int[] bd, int[] tl, int[] td, int[] hp, ZStream z)
    pub fn inflate_trees_dynamic(&mut self, _nl: i32, _nd: i32, _c: &mut [i32], _bl: &mut [i32], _bd: &mut [i32], _tl: &mut [i32], _td: &mut [i32], _hp: &mut [i32], _z: &mut dyn std::any::Any) -> i32 {
      // Java: this.initWorkArea(288);
      // Java: this.hn[0] = 0;
      // Java: int result = this.huft_build(c, 0, nl, 257, cplens, cplext, tl, bl, hp, this.hn, this.v);
      // Java: if (result == 0 && bl[0] != 0)
      {
         // Java: this.initWorkArea(288);
         // Java: result = this.huft_build(c, nl, nd, 0, cpdist, cpdext, td, bd, hp, this.hn, this.v);
         // Java: if (result != 0 || bd[0] == 0 && nl > 257)
         {
            // Java: if (result == -3)
            {
               // Java: z.msg = "oversubscribed distance tree";
            }
            // Java: else if (result == -5)
            {
               // Java: z.msg = "incomplete distance tree";
               // Java: result = -3;
            }
            // Java: else if (result != -4)
            {
               // Java: z.msg = "empty distance tree with lengths";
               // Java: result = -3;
            }
            // Java: return result;
         }
         // Java: else
         {
            // Java: return 0;
         }
      }
      // Java: else
      {
         // Java: if (result == -3)
         {
            // Java: z.msg = "oversubscribed literal/length tree";
         }
         // Java: else if (result != -4)
         {
            // Java: z.msg = "incomplete literal/length tree";
            // Java: result = -3;
         }
         // Java: return result;
      }
      todo!()
    }

    // Java: static int inflate_trees_fixed(int[] bl, int[] bd, int[][] tl, int[][] td, ZStream z)
    pub fn inflate_trees_fixed(&self, _bl: &mut [i32], _bd: &mut [i32], _tl: &mut [Vec<i32>], _td: &mut [Vec<i32>], _z: &mut dyn std::any::Any) -> i32 {
      // Java: bl[0] = 9;
      // Java: bd[0] = 5;
      // Java: tl[0] = fixed_tl;
      // Java: td[0] = fixed_td;
      // Java: return 0;
      todo!()
    }

    // Java: void initWorkArea(int vsize)
    pub fn initWorkArea(&mut self, _vsize: i32) {
      // Java: if (this.hn == null)
      {
         // Java: this.hn = new int[1];
         // Java: this.v = new int[vsize];
         // Java: this.c = new int[16];
         // Java: this.r = new int[3];
         // Java: this.u = new int[15];
         // Java: this.x = new int[16];
      }
      // Java: if (this.v.length < vsize)
      {
         // Java: this.v = new int[vsize];
      }
      // Java: for(int i = 0; i < vsize; ++i)
      {
         // Java: this.v[i] = 0;
      }
      // Java: for(int i = 0; i < 16; ++i)
      {
         // Java: this.c[i] = 0;
      }
      // Java: for(int i = 0; i < 3; ++i)
      {
         // Java: this.r[i] = 0;
      }
      // Java: System.arraycopy(this.c, 0, this.u, 0, 15);
      // Java: System.arraycopy(this.c, 0, this.x, 0, 16);
    }

}
