
println("Caesar Cipher \n")
println("Commands: ")
println("\t - [1] Encrypt ")
println("\t - [2] Decrypt ")

println("\nWhat do you want to do?") 
print("Command: ")
decision = parse(Int8, readline())

alphabet = split("abcdefghijklmnopqrstuvwxyz ", "")

if decision != 1 && decision != 2
    println("Invalid Command!")

elseif decision == 1
    println("\nEnter the text you want to encrypt")
    print("Text: ")
    text = readline()
    text = split(text, "")
    encrypted_array = []

    print("Key: ")
    key = readline()

    for i in 1:length(text)

        index = findfirst(x-> x == text[i], alphabet)

        if text[i] == " "
            encrypted_array.append(index)
        else
            encrypted_array.append((index + key) % 26)
        end
    
    end
    
    println("\nEncrypted Text: $(join(encrypted_array))")
    println("Key $key")
    
end

    # for i in 1:length(text)
    #     text_array.append(char[])

