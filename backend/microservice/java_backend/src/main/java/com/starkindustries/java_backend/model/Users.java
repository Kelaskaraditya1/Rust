package com.starkindustries.java_backend.model;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@AllArgsConstructor
@NoArgsConstructor
@Builder
public class Users {

    public String userId;
    public String name;
    public String email;
    public String contact;
    public String username;
    public String password;
    
}
