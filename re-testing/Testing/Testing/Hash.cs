namespace CodecTest;

public static class Hash
{
    private static readonly byte[] Salt =
    {
        0xE7, 0xB2, 0x87, 0x76, 0xD1, 0xEF, 0x3F, 0xE0,
        0xE0, 0x68, 0xE4, 0x1D, 0xD5, 0xC9, 0x57, 0xFF
    };
    
    public static void GenerateHash(Span<byte> encryptedData, Span<byte> destination)
    {
        if (destination.Length != 16)
            throw new ArgumentException("Destination needs to be 16 bytes long");

        // in the game a static array is hashed before the encrypted data, but it's always 0 length so
        // no point implementing it at this point
        CalculateHash(encryptedData, destination);

        Span<byte> hash2 = stackalloc byte[16];
        
        // 0-3 are always 0
        hash2[..4].Fill(0);
        
        // 4-7 is where we'd put static hash len, but the game always sets it to 0 so we won't bother
        // it uses the same shifts as the code for encrypted data length below
        hash2[4..8].Fill(0);
        
        // 8-11 are always 0
        hash2[8..12].Fill(0);

        unchecked
        {
            hash2[12] = (byte)((uint)(8 * encryptedData.Length) >> 24);
            hash2[13] = (byte)((uint)encryptedData.Length >> 13);
            hash2[14] = (byte)((uint)encryptedData.Length >> 5);
            hash2[15] = (byte)(8 * (uint)encryptedData.Length);
        }

        CalculateHash(hash2, destination);
    }

    // 0x7FF609E59D90
    private static void CalculateHash(Span<byte> bytes, Span<byte> destination)
    {
        int srcPos = 0;
        Span<byte> saltedBytes = stackalloc byte[16];
        
        if (bytes.Length >= 16)
        {
            for (int i = bytes.Length >> 4; i > 0; --i)
            {
                for (int j = 0; j < 16; ++j)
                {
                    destination[j] ^= bytes[srcPos + j];
                }
                
                srcPos += 16;

                SaltBytes(destination, saltedBytes);
                saltedBytes.CopyTo(destination);
            }
        }

        if (srcPos < bytes.Length)
        {
            int remaining = 16 - (bytes.Length - srcPos);

            bytes[srcPos..bytes.Length].CopyTo(saltedBytes);
            srcPos += bytes.Length - srcPos;
            saltedBytes[(16 - remaining)..].Fill(0);

            for (int i = 0; i < 16; ++i)
            {
                destination[i] ^= saltedBytes[i];
            }

            SaltBytes(destination, saltedBytes);
            saltedBytes.CopyTo(destination);
        }
    }

    // 0x7FF672EF9C20
    private static void SaltBytes(Span<byte> bytes, Span<byte> destination)
    {
        Span<byte> saltCopy = stackalloc byte[16];
    
        destination.Fill(0); // fill destination with zeroes as it might be reused
        Salt.CopyTo(saltCopy);

        for (int i = 0; i < 16; ++i)
        {
            for (int j = 0; j < 8; ++j)
            {
                int v8 = (7 - j) switch
                {
                    0 => 1,
                    1 => 2,
                    2 => 4,
                    3 => 8,
                    4 => 16,
                    5 => 32,
                    6 => 64,
                    7 => 128,
                    _ => throw new InvalidOperationException("unreachable")
                };

                if ((v8 & bytes[i]) != 0)
                {
                    for (int k = 0; k < 16; ++k)
                    {
                        destination[k] ^= saltCopy[k];
                    }
                }

                var lastSaltByte = saltCopy[15];
                ShiftSaltBytes(saltCopy);

                if ((lastSaltByte & 1) != 0)
                {
                    saltCopy[0] ^= 0xE1;
                }
            }
        }
    }

    // 0x7FF609E597C0
    private static void ShiftSaltBytes(Span<byte> bytes)
    {
        byte v2 = bytes[11];
        uint v3 = (uint)(bytes[15] | ((bytes[14] | ((bytes[13] | (bytes[12] << 8)) << 8)) << 8)) >> 1;
        uint v4 = v3 | 0x80000000;
        
        if ((v2 & 1) == 0)
            v4 = v3;
        
        bytes[12] = (byte)((v4 >> 24) & 0xFF);
        bytes[13] = (byte)((v4 >> 16) & 0xFF);
        bytes[14] = (byte)((v4 >> 8) & 0xFF);
        bytes[15] = (byte)(v4 & 0xFF);

        uint v5 = ((uint)(bytes[11] | ((bytes[10] | ((bytes[9] | (bytes[8] << 8)) << 8)) << 8)) >> 1) | 0x80000000;
        
        if ((bytes[7] & 1) == 0)
            v5 = (uint)(bytes[11] | ((bytes[10] | ((bytes[9] | (bytes[8] << 8)) << 8)) << 8)) >> 1;
        
        bytes[8] = (byte)(v5 >> 24);
        bytes[9] = (byte)(v5 >> 16);
        bytes[10] = (byte)(v5 >> 8);
        bytes[11] = (byte)v5;

        uint v6 = ((uint)(bytes[7] | ((bytes[6] | ((bytes[5] | (bytes[4] << 8)) << 8)) << 8)) >> 1) | 0x80000000;
        
        if ((bytes[3] & 1) == 0)
            v6 = (uint)(bytes[7] | ((bytes[6] | ((bytes[5] | (bytes[4] << 8)) << 8)) << 8)) >> 1;
        
        bytes[4] = (byte)(v6 >> 24);
        bytes[5] = (byte)(v6 >> 16);
        bytes[6] = (byte)(v6 >> 8);
        bytes[7] = (byte)v6;

        uint v7 = (uint)(bytes[3] | ((bytes[2] | ((bytes[1] | (bytes[0] << 8)) << 8)) << 8)) >> 1;
        bytes[0] = (byte)(v7 >> 24);
        bytes[1] = (byte)(v7 >> 16);
        bytes[2] = (byte)(v7 >> 8);
        bytes[3] = (byte)v7;
    }
}