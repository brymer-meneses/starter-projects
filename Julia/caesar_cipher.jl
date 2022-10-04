
println("Caesar Cipher \n")
println("Commands: ")
println("\t - [1] Encrypt ")
println("\t - [2] Decrypt ")

println("\nWhat do you want to do?") 
print("Command: ")
decision = parse(Int8, readline())

alphabet = split("abcdefghijklmnopqrstuvwxyz", "")

if decision != 1 && decision != 2
    println("Invalid Command!")

elseif decision == 1
    println("\nEnter the text you want to encrypt")
    print("Text: ")
    text = readline()
    text = split(text, "")
    encrypted_array = Int8[]
    encrypted_text = Any[]

    print("Key: ")
    key = parse(Int8,readline())

    for i in 1:length(text)

        index = findfirst(x-> x == text[i], alphabet)

        if text[i] == " "
            append!(encrypted_text, " ")
        else
            append!(encrypted_array, (index + key) % 26)
        end
    
    end

    for i in 1:length(encrypted_array)
        if encrypted_array[i] == 0 
            index = 26
        else 
            index = i 
        end

        append!(encrypted_text, alphabet[index])
    end
    
    println("\nEncrypted Text: $(encrypted_text)")
    println(join(encrypted_text))
    println("Key $key")
    
end

    # for i in 1:length(text)
    #     text_array.append(char[])

