using System.Text;
using CodecTest;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

internal class Program
{
    // Test message
    private const string B64Message = "pXCLBQnhkvI9/EUOsL6neoyaAocGq8q/zWViLKd+eO7v0azvPkUhAHGk3oC9cTnb1siObGwH73IWLdNPxck3ratY4DeIH1/TT+bT0Js3+KqyiCIboJKHIhP8UHkhnykeybdhGRTLyM2vTDRZ9Wd31zn5XkDBopdRYEIZ4C4XF6t/ux5cVeZFBbW5aWn9auSzF70jsm2Ffbuy2pU2WYzaZG5yIycMal8XzOJYEtjWBwaft2BCp4Lz/gHmqTwNoX+Fuv/qWQDzy2LDkY3NjfR2f7WjJHiV0m1ngdYealY4KvbLbc6qn2L2Qo2t/1t0+Ygz8z0QwO1ySLL81VGu0VWD+H9TO1J1TnyJ9w==";

    private static readonly byte[] AesSecret =
    {
        0x4C, 0x48, 0x77, 0x55, 0x47, 0x6E, 0x6B, 0x74,
        0x43, 0x6C, 0x4E, 0x76, 0x39, 0x55, 0x6F, 0x63,
        0x31, 0x47, 0x71, 0x7A, 0x63, 0x63, 0x62, 0x68,
        0x72, 0x64, 0x61, 0x4A, 0x33, 0x41, 0x06, 0x0A,
    };

    public static void Main(string[] args)
    {
        var messageBytes = Convert.FromBase64String(B64Message).AsMemory();
        Span<byte> ivKey = stackalloc byte[16];
        messageBytes.Span[..12].CopyTo(ivKey);
        var encryptedData = messageBytes.Span[12..^16];
        var encryptedHash = messageBytes.Span[^16..];

        ivKey[15] = 2; // final iv key byte is 2 when encrypting/decrypting data
        string decryptedData = EncryptDecryptString(encryptedData, AesSecret, ivKey);

        var jObject = JsonConvert.DeserializeObject<JObject>(decryptedData);
        decryptedData = JsonConvert.SerializeObject(jObject, Formatting.Indented);

        Span<byte> hash = stackalloc byte[16];
        Hash.GenerateHash(encryptedData, hash);

        ivKey[15] = 1; // final iv key byte is 1 when encrypting/decrypting hash
        var compareEncryptedHash = EncryptDecryptBytes(hash, AesSecret, ivKey);

        Console.WriteLine(decryptedData);

        if (CompareSpans(encryptedHash, compareEncryptedHash))
        {
            Console.WriteLine("The data is valid, hashes match");
        }
        else
        {
            Console.WriteLine($"The data is NOT valid, hashes do not match");
        }
    }

    private static bool CompareSpans(Span<byte> a, Span<byte> b)
    {
        if (a.Length != b.Length)
            return false;

        for (int i = 0; i < a.Length; ++i)
        {
            if (a[i] != b[i])
                return false;
        }

        return true;
    }

    private static string EncryptDecryptString(Span<byte> data, Span<byte> secret, Span<byte> iv)
    {
        return Encoding.UTF8.GetString(EncryptDecryptBytes(data, secret, iv));
    }

    private static byte[] EncryptDecryptBytes(Span<byte> data, Span<byte> secret, Span<byte> iv)
    {
        using var aes = new AesCtr();
        aes.Key = secret.ToArray();
        aes.IV = iv.ToArray();

        var decryptor = aes.CreateEncryptor(); // CTR uses same method to encrypt and decrypt
        byte[] outputBuffer = new byte[data.Length];

        decryptor.TransformBlock(data.ToArray(), 0, data.Length, outputBuffer, 0);

        return outputBuffer;
    }
    
    internal static string ByteArrayToString(Span<byte> bytes)
    {
        StringBuilder hex = new StringBuilder(bytes.Length * 3);
        foreach (byte b in bytes)
            hex.AppendFormat("{0:X2} ", b);
        return hex.ToString().TrimEnd();
    }
}