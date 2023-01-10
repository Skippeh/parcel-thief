// Taken and modified from https://stackoverflow.com/a/29562965/3559163

using System.Security.Cryptography;

namespace CodecTest;

public class AesCtr : Aes
{
    private readonly Aes aes;

    public AesCtr()
    {
        aes = Create();
        aes.Mode = CipherMode.ECB;
        aes.Padding = PaddingMode.None;
    }

    public override byte[] IV
    {
        get => aes.IV;
        set => aes.IV = value;
    }

    public override byte[] Key
    {
        get => aes.Key;
        set => aes.Key = value;
    }

    public override int KeySize
    {
        get => aes.KeySize;
        set => aes.KeySize = value;
    }

    public override CipherMode Mode
    {
        get => aes.Mode;
        set
        {
            if (value != CipherMode.ECB)
            {
                throw new CryptographicException("Only ECB mode is supported");
            }
        }
    }

    public override PaddingMode Padding
    {
        get => aes.Padding;
        set
        {
            if (value != PaddingMode.None)
            {
                throw new CryptographicException("Padding is not supported");
            }
        }
    }

    public override int BlockSize
    {
        get => 8;
        set
        {
            if (value != 8)
            {
                throw new CryptographicException("The only supported block size is 8");
            }
        }
    }

    public override KeySizes[] LegalBlockSizes
    {
        get
        {
            return new[] { new KeySizes(BlockSize, BlockSize, 0) };
        }
    }

    public override int FeedbackSize
    {
        get => aes.FeedbackSize;
        set
        {
            if (FeedbackSize != aes.FeedbackSize)
            {
                throw new CryptographicException();
            }
        }
    }

    public override ICryptoTransform CreateDecryptor()
    {
        // Note that we always use the Aes.CreateEncryptor, even for
        // decrypting, because we only have to "rebuild" the encrypted
        // CTR nonce.
        return CreateEncryptor();
    }

    public override ICryptoTransform CreateDecryptor(byte[] key, byte[]? iv)
    {
        // Note that we always use the Aes.CreateEncryptor, even for
        // decrypting, because we only have to "rebuild" the encrypted
        // CTR nonce.
        return CreateEncryptor(key, iv);
    }

    public override ICryptoTransform CreateEncryptor()
    {
        return new CtrStreamCipher(aes.CreateEncryptor(), IV);
    }

    public override ICryptoTransform CreateEncryptor(byte[] key, byte[]? iv)
    {
        if (key == null)
        {
            throw new ArgumentNullException(nameof(key));
        }

        if (!ValidKeySize(key.Length * 8))
        {
            throw new ArgumentException("key");
        }

        if (iv == null)
        {
            throw new ArgumentNullException(nameof(iv));
        }

        if (iv.Length * 8 != BlockSizeValue)
        {
            throw new ArgumentException("iv");
        }

        return new CtrStreamCipher(aes.CreateEncryptor(key, iv), iv);
    }

    public override void GenerateIV()
    {
        aes.GenerateIV();
    }

    public override void GenerateKey()
    {
        aes.GenerateKey();
    }

    protected override void Dispose(bool disposing)
    {
        try
        {
            if (disposing)
            {
                aes.Dispose();
            }
        }
        finally
        {
            base.Dispose(disposing);
        }
    }

    public sealed class CtrStreamCipher : ICryptoTransform
    {
        private ICryptoTransform? transform;

        private byte[]? iv;
        private byte[]? encryptedIv = new byte[16];
        private int encryptedIvOffset;

        public CtrStreamCipher(ICryptoTransform transform, byte[] iv)
        {
            this.transform = transform;

            // Note that in this implementation the IV/Nonce and the 
            // Counter described by http://en.wikipedia.org/wiki/Block_cipher_mode_of_operation#Counter_.28CTR.29
            // are additioned together in a single IV, that then is
            // incremented by 1 in a "big-endian" mode.
            this.iv = (byte[])iv.Clone();
            this.transform.TransformBlock(this.iv, 0, this.iv.Length, encryptedIv, 0);
        }

        public bool CanReuseTransform => true;

        public bool CanTransformMultipleBlocks => true;

        public int InputBlockSize => 1;

        public int OutputBlockSize => 1;

        public int TransformBlock(byte[] inputBuffer, int inputOffset, int inputCount, byte[] outputBuffer, int outputOffset)
        {
            int count = Math.Min(inputCount, outputBuffer.Length - outputOffset);

            for (int i = 0; i < count; i++)
            {
                if (encryptedIvOffset == encryptedIv!.Length)
                {
                    IncrementNonceAndResetOffset();
                }

                outputBuffer[outputOffset + i] = (byte)(inputBuffer[inputOffset + i] ^ encryptedIv[encryptedIvOffset]);
                encryptedIvOffset++;
            }

            return count;
        }

        public byte[] TransformFinalBlock(byte[] inputBuffer, int inputOffset, int inputCount)
        {
            // This method can be reused. There is no "final block" in
            // CTR mode, because characters are encrypted one by one
            byte[] outputBuffer = new byte[inputCount];
            TransformBlock(inputBuffer, inputOffset, inputCount, outputBuffer, 0);
            return outputBuffer;
        }

        public void Dispose()
        {
            if (transform != null)
            {
                transform.Dispose();
                transform = null;
                iv = null;
                encryptedIv = null;
            }

            GC.SuppressFinalize(this);
        }

        private void IncrementNonceAndResetOffset()
        {
            int i = iv!.Length - 1;

            do
            {
                unchecked
                {
                    iv[i]++;
                }

                if (iv[i] != 0 || i == 0)
                {
                    break;
                }

                i--;
            }
            while (true);

            transform!.TransformBlock(iv, 0, iv.Length, encryptedIv!, 0);
            encryptedIvOffset = 0;
        }
    }
}