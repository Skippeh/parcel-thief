function useStorage<T>(storageType: "local" | "session", key: string) {
  const storage = getStorage(storageType);

  const get = (): T | null => {
    const value = storage.getItem(key);

    if (value == null) {
      return null;
    }

    return JSON.parse(value);
  };

  const set = (value: T) => {
    storage.setItem(key, JSON.stringify(value));
  };

  const remove = () => {
    storage.removeItem(key);
  };

  return { get, set, remove };
}

function getStorage(storageType: "local" | "session"): Storage {
  let storage: Storage;

  switch (storageType) {
    case "local":
      storage = localStorage;
      break;
    case "session":
      storage = sessionStorage;
      break;
    default:
      throw new Error("Invalid storage type specified");
  }

  return storage;
}

export default useStorage;
