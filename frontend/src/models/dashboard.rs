pub struct DashboardData {
    pub nama_pengguna: &'static str,
    pub ringkasan: [KartuRingkasan; 4],
    pub pergerakan_stok: [BatangGrafik; 12],
    pub distribusi_gudang: [DistribusiGudang; 3],
    pub barang_terbaru: [BarangTerbaru; 4],
}

pub struct KartuRingkasan {
    pub label: &'static str,
    pub nilai: &'static str,
    pub keterangan: &'static str,
    pub ikon: &'static str,
    pub warna: &'static str,
}

pub struct BatangGrafik {
    pub label: &'static str,
    pub tinggi: &'static str,
    pub warna: &'static str,
}

pub struct DistribusiGudang {
    pub nama: &'static str,
    pub jumlah: &'static str,
    pub lebar: &'static str,
    pub warna: &'static str,
}

pub struct BarangTerbaru {
    pub nama: &'static str,
    pub kode: &'static str,
    pub gudang: &'static str,
    pub stok: &'static str,
    pub harga: &'static str,
    pub total_nilai: &'static str,
    pub status: &'static str,
    pub kelas_status: &'static str,
}

impl DashboardData {
    pub fn data_contoh() -> Self {
        Self {
            nama_pengguna: "Raka",
            ringkasan: [
                KartuRingkasan {
                    label: "Total Barang",
                    nilai: "1.284",
                    keterangan: "Barang terdaftar",
                    ikon: "kotak",
                    warna: "biru",
                },
                KartuRingkasan {
                    label: "Total Stok",
                    nilai: "8.642",
                    keterangan: "Unit tersedia",
                    ikon: "tumpukan",
                    warna: "ungu",
                },
                KartuRingkasan {
                    label: "Total Gudang",
                    nilai: "3",
                    keterangan: "Lokasi penyimpanan",
                    ikon: "gudang",
                    warna: "hijau",
                },
                KartuRingkasan {
                    label: "Stok Menipis",
                    nilai: "18",
                    keterangan: "Perlu diperiksa",
                    ikon: "peringatan",
                    warna: "jingga",
                },
            ],
            pergerakan_stok: [
                BatangGrafik {
                    label: "01 Nov",
                    tinggi: "52",
                    warna: "hijau",
                },
                BatangGrafik {
                    label: "",
                    tinggi: "68",
                    warna: "biru",
                },
                BatangGrafik {
                    label: "",
                    tinggi: "43",
                    warna: "biru",
                },
                BatangGrafik {
                    label: "",
                    tinggi: "60",
                    warna: "hijau",
                },
                BatangGrafik {
                    label: "10 Nov",
                    tinggi: "49",
                    warna: "biru",
                },
                BatangGrafik {
                    label: "",
                    tinggi: "78",
                    warna: "biru",
                },
                BatangGrafik {
                    label: "",
                    tinggi: "58",
                    warna: "hijau",
                },
                BatangGrafik {
                    label: "",
                    tinggi: "84",
                    warna: "biru",
                },
                BatangGrafik {
                    label: "20 Nov",
                    tinggi: "57",
                    warna: "biru",
                },
                BatangGrafik {
                    label: "",
                    tinggi: "75",
                    warna: "hijau",
                },
                BatangGrafik {
                    label: "",
                    tinggi: "65",
                    warna: "biru",
                },
                BatangGrafik {
                    label: "30 Nov",
                    tinggi: "100",
                    warna: "biru",
                },
            ],
            distribusi_gudang: [
                DistribusiGudang {
                    nama: "Gudang 1",
                    jumlah: "486 barang",
                    lebar: "78",
                    warna: "biru",
                },
                DistribusiGudang {
                    nama: "Gudang 2",
                    jumlah: "421 barang",
                    lebar: "68",
                    warna: "hijau",
                },
                DistribusiGudang {
                    nama: "Gudang 3",
                    jumlah: "377 barang",
                    lebar: "61",
                    warna: "ungu",
                },
            ],
            barang_terbaru: [
                BarangTerbaru {
                    nama: "Mouse",
                    kode: "BRG-001",
                    gudang: "Gudang 1",
                    stok: "24",
                    harga: "Rp185.000",
                    total_nilai: "Rp4.440.000",
                    status: "Tersedia",
                    kelas_status: "tersedia",
                },
                BarangTerbaru {
                    nama: "Kabel HDMI",
                    kode: "BRG-002",
                    gudang: "Gudang 1",
                    stok: "8",
                    harga: "Rp45.000",
                    total_nilai: "Rp360.000",
                    status: "Menipis",
                    kelas_status: "menipis",
                },
                BarangTerbaru {
                    nama: "Keyboard",
                    kode: "BRG-003",
                    gudang: "Gudang 2",
                    stok: "0",
                    harga: "Rp220.000",
                    total_nilai: "Rp0",
                    status: "Habis",
                    kelas_status: "habis",
                },
                BarangTerbaru {
                    nama: "Printer",
                    kode: "BRG-004",
                    gudang: "Gudang 3",
                    stok: "12",
                    harga: "Rp1.250.000",
                    total_nilai: "Rp15.000.000",
                    status: "Tersedia",
                    kelas_status: "tersedia",
                },
            ],
        }
    }
}
