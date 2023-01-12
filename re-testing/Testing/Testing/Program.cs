using System.Security.Cryptography;
using System.Text;
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
        var encryptedData = messageBytes.Span[12..^16];
        var tag = messageBytes.Span[^16..];

        var aesGcm = new AesGcm(AesSecret);
        Span<byte> decryptedData = stackalloc byte[encryptedData.Length];
        aesGcm.Decrypt(messageBytes[..12].Span, encryptedData, tag, decryptedData, null);
        string decryptedString = Encoding.UTF8.GetString(decryptedData);
        
        var jObject = JsonConvert.DeserializeObject<JObject>(decryptedString);
        decryptedString = JsonConvert.SerializeObject(jObject, Formatting.Indented);

        Console.WriteLine(decryptedString);
    }
    
    internal static string ByteArrayToString(Span<byte> bytes)
    {
        StringBuilder hex = new StringBuilder(bytes.Length * 3);
        foreach (byte b in bytes)
            hex.AppendFormat("{0:X2} ", b);
        return hex.ToString().TrimEnd();
    }
}